// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
package protocol

import (
	"context"
	"fmt"
	"log"
	"net/url"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"

	"github.com/Azure/iot-operations-sdks/go/protocol"
	"github.com/Azure/iot-operations-sdks/go/protocol/errors"
	"github.com/BurntSushi/toml"
	"github.com/relvacode/iso8601"
	"github.com/stretchr/testify/require"
	"gopkg.in/yaml.v3"
)

func RunTelemetrySenderTests(t *testing.T) {
	var telemetrySenderDefaultInfo DefaultTestCase

	_, err := toml.DecodeFile(
		"../../../eng/test/test-cases/Protocol/TelemetrySender/defaults.toml",
		&telemetrySenderDefaultInfo,
	)
	if err != nil {
		panic(err)
	}

	TestCaseDefaultInfo = &telemetrySenderDefaultInfo
	TestCaseDefaultSerializer = &telemetrySenderDefaultInfo.Prologue.Sender.Serializer

	files, err := filepath.Glob(
		"../../../eng/test/test-cases/Protocol/TelemetrySender/*.yaml",
	)
	if err != nil {
		log.Fatal(err)
	}

	for ix, f := range files {
		testName, _ := strings.CutSuffix(filepath.Base(f), ".yaml")
		t.Run(testName, func(t *testing.T) {
			runOneTelemetrySenderTest(t, ix, testName, f)
		})
	}
}

func runOneTelemetrySenderTest(
	t *testing.T,
	testCaseIndex int,
	testName string,
	fileName string,
) {
	pendingTestCases := []string{
		"TelemetrySenderPubAckDroppedByDisconnection_ReconnectAndSuccess", // hangs intermittently
		"TelemetrySenderPubAckFailure_ThrowsException",                    // perhaps related to https://github.com/eclipse/paho.golang/issues/216
	}

	testCaseYaml, err := os.ReadFile(fileName)
	if err != nil {
		log.Fatal(err)
	}

	//	t.Parallel()

	var testCase TestCase
	err = yaml.Unmarshal(testCaseYaml, &testCase)
	if err != nil {
		log.Fatal(err)
	}

	if slices.Contains(testCase.Requires, Unobtanium) ||
		slices.Contains(testCase.Requires, ExplicitDefault) {
		t.Skipf(
			"Skipping test %s because it requires an unavailable feature",
			testName,
		)
	}

	if slices.Contains(pendingTestCases, testName) {
		t.Skipf(
			"Skipping test %s because it requires a feature which has not yet been implemented",
			testName,
		)
	}

	var mqttClientID string
	if testCase.Prologue.MqttConfig.ClientID != nil {
		mqttClientID = *testCase.Prologue.MqttConfig.ClientID
	} else {
		mqttClientID = fmt.Sprintf("SenderTestClient%d", testCaseIndex)
	}

	stubBroker, sessionClient := getStubAndSessionClient(t, mqttClientID)

	for _, ackKind := range testCase.Prologue.PushAcks.Publish {
		stubBroker.EnqueuePubAck(ackKind)
	}

	for _, ackKind := range testCase.Prologue.PushAcks.Subscribe {
		stubBroker.EnqueueSubAck(ackKind)
	}

	for _, ackKind := range testCase.Prologue.PushAcks.Unsubscribe {
		stubBroker.EnqueueUnsubAck(ackKind)
	}

	telemetrySenders := make(map[string]*TestingTelemetrySender)

	for ix, tcs := range testCase.Prologue.Senders {
		var catch *TestCaseCatch
		if ix == len(testCase.Prologue.Senders)-1 {
			catch = testCase.Prologue.Catch
		}

		sender := getTelemetrySender(t, sessionClient, stubBroker, &tcs, catch)
		if sender != nil {
			telemetrySenders[*tcs.TelemetryName] = sender
		}
	}

	sendChan := make(chan error)

	for _, action := range testCase.Actions {
		switch action.Kind {
		case SendTelemetry:
			sendTelemetry(
				t,
				action.AsSendTelemetry(),
				telemetrySenders,
				sendChan,
			)
		case AwaitSend:
			awaitSend(t, action.AsAwaitSend(), sendChan)
		case AwaitPublish:
			awaitPublishTelemetry(
				t,
				action.AsAwaitPublish(),
				stubBroker,
				map[int][]byte{},
			)
		case Disconnect:
			stubBroker.Disconnect()
		}
	}

	close(sendChan)

	for err := range sendChan {
		require.NoErrorf(
			t,
			err,
			"Unexpected error returned when awaiting TelemetrySender.Send()",
		)
	}

	if testCase.Epilogue.PublicationCount != nil {
		require.Equal(
			t,
			*testCase.Epilogue.PublicationCount,
			stubBroker.PublicationCount,
		)
	}

	for ix, publishedMessage := range testCase.Epilogue.PublishedMessages {
		checkPublishedTelemetry(t, ix, &publishedMessage, stubBroker)
	}
}

func getTelemetrySender(
	t *testing.T,
	sessionClient protocol.MqttClient,
	stubBroker *StubBroker,
	tcs *TestCaseSender,
	catch *TestCaseCatch,
) *TestingTelemetrySender {
	options := []protocol.TelemetrySenderOption{
		protocol.WithTopicTokens(tcs.TopicTokenMap),
	}

	if tcs.TopicNamespace != nil {
		options = append(
			options,
			protocol.WithTopicNamespace(*tcs.TopicNamespace),
		)
	}

	sender, err := NewTestingTelemetrySender(
		sessionClient,
		&tcs.Serializer,
		tcs.TelemetryTopic,
		options...)

	if catch == nil {
		require.NoErrorf(
			t,
			err,
			"Unexpected error returned when initializing TelemetrySender: %s",
			err,
		)
	} else {
		if err == nil {
			err = sender.base.Send(context.Background(), *TestCaseDefaultInfo.Actions.SendTelemetry.GetTelemetryValue())
			stubBroker.AwaitPublish()
		}

		require.Errorf(t, err, "Expected %s error, but no error returned when initializing TelemetrySender", catch.ErrorKind)
		CheckError(t, *catch, err)
	}

	return sender
}

func sendTelemetry(
	t *testing.T,
	actionSendTelemetry *TestCaseActionSendTelemetry,
	telemetrySenders map[string]*TestingTelemetrySender,
	sendChan chan error,
) {
	telem := *actionSendTelemetry.TelemetryValue

	options := []protocol.SendOption{}
	options = append(
		options,
		protocol.WithTimeout(actionSendTelemetry.Timeout.ToDuration()),
		protocol.WithTopicTokens(actionSendTelemetry.TopicTokenMap),
	)

	if actionSendTelemetry.Qos != nil && *actionSendTelemetry.Qos != 1 {
		t.Skipf(
			"Skipping test because TelemetrySender does not support settable QoS",
		)
	}

	if actionSendTelemetry.Metadata != nil {
		for key, val := range *actionSendTelemetry.Metadata {
			options = append(
				options,
				protocol.WithMetadata{
					key: val,
				},
			)
		}
	}

	if actionSendTelemetry.CloudEvent != nil {
		cloudEvent := protocol.CloudEvent{}

		if actionSendTelemetry.CloudEvent.Source != nil {
			source, err := url.Parse(*actionSendTelemetry.CloudEvent.Source)
			if err != nil {
				go func() { sendChan <- getCloudEventError("Source", *actionSendTelemetry.CloudEvent.Source, "URL", err) }()
				return
			}
			cloudEvent.Source = source
		}

		if actionSendTelemetry.CloudEvent.SpecVersion != nil {
			cloudEvent.SpecVersion = *actionSendTelemetry.CloudEvent.SpecVersion
		}

		if actionSendTelemetry.CloudEvent.Type != nil {
			cloudEvent.Type = *actionSendTelemetry.CloudEvent.Type
		}

		if actionSendTelemetry.CloudEvent.ID != nil {
			cloudEvent.ID = *actionSendTelemetry.CloudEvent.ID
		}

		if time, ok := actionSendTelemetry.CloudEvent.Time.(string); ok {
			isoTime, err := iso8601.ParseString(time)
			if err != nil {
				go func() { sendChan <- getCloudEventError("Time", time, "ISO 8601", err) }()
				return
			}
			cloudEvent.Time = isoTime
		}

		if subject, ok := actionSendTelemetry.CloudEvent.Subject.(string); ok {
			cloudEvent.Subject = subject
		}

		if dataSchema, ok := actionSendTelemetry.CloudEvent.DataSchema.(string); ok {
			dataSchemaURL, err := url.Parse(dataSchema)
			if err != nil {
				go func() { sendChan <- getCloudEventError("DataSchema", dataSchema, "URL", err) }()
				return
			}
			cloudEvent.DataSchema = dataSchemaURL
		}

		options = append(
			options,
			protocol.WithCloudEvent(&cloudEvent),
		)
	}

	tcs := telemetrySenders[*actionSendTelemetry.TelemetryName]

	go func() {
		err := tcs.base.Send(
			context.Background(),
			telem,
			options...)
		sendChan <- err
	}()
}

func awaitSend(
	t *testing.T,
	actionAwaitSend *TestCaseActionAwaitSend,
	sendChan chan error,
) {
	err := <-sendChan

	if actionAwaitSend.Catch == nil {
		require.NoErrorf(
			t,
			err,
			"Unexpected error returned when awaiting TelemetrySender.Send()",
		)
	} else {
		require.Errorf(t, err, "Expected %s error, but no error returned when awaiting TelemetrySender.Send()", actionAwaitSend.Catch.ErrorKind)
		CheckError(t, *actionAwaitSend.Catch, err)
	}
}

func awaitPublishTelemetry(
	_ *testing.T,
	_ *TestCaseActionAwaitPublish,
	stubBroker *StubBroker,
	_ map[int][]byte,
) {
	stubBroker.AwaitPublish()
}

func checkPublishedTelemetry(
	t *testing.T,
	sequenceIndex int,
	publishedMessage *TestCasePublishedMessage,
	stubBroker *StubBroker,
) {
	msg, ok := stubBroker.GetIndexedPublishedMessage(sequenceIndex)
	require.True(t, ok)

	if publishedMessage.Topic != nil {
		require.Equal(t, *publishedMessage.Topic, msg.Topic)
	}

	if publishedMessage.Payload == nil {
		require.Empty(t, msg.Payload)
	} else if payload, ok := publishedMessage.Payload.(string); ok {
		require.Equal(t, payload, string(msg.Payload))
	}

	if publishedMessage.ContentType != nil {
		require.Equal(
			t,
			*publishedMessage.ContentType,
			msg.Properties.ContentType,
		)
	}

	if publishedMessage.FormatIndicator != nil {
		require.Equal(
			t,
			*publishedMessage.FormatIndicator,
			*msg.Properties.PayloadFormat,
		)
	}

	for key, val := range publishedMessage.Metadata {
		propVal, ok := getUserProperty(t, msg, key)
		if val != nil {
			require.True(t, ok)
			require.Equal(t, *val, propVal)
		} else {
			require.False(t, ok)
		}
	}

	if publishedMessage.SourceID != nil {
		sourceID, ok := getUserProperty(t, msg, SourceID)
		require.True(t, ok)
		require.Equal(t, *publishedMessage.SourceID, sourceID)
	}

	if publishedMessage.Expiry != nil {
		require.Equal(
			t,
			*publishedMessage.Expiry,
			*msg.Properties.MessageExpiry,
		)
	}
}

func getCloudEventError(
	fieldName string,
	propValue string,
	parseType string,
	err error,
) error {
	return &errors.Client{
		Message: fmt.Sprintf(
			"cloud event %s not parsable as %s",
			fieldName,
			parseType,
		),
		Kind: errors.ConfigurationInvalid{
			PropertyName:  "CloudEvent",
			PropertyValue: propValue,
		},
		Nested:  err,
		Shallow: true,
	}
}
