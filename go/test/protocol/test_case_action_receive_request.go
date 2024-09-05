package protocol

import (
	"gopkg.in/yaml.v3"
)

type testCaseActionReceiveRequest struct {
	Topic               *string           `yaml:"topic"`
	Payload             *string           `yaml:"payload"`
	BypassSerialization bool              `yaml:"bypass-serialization"`
	ContentType         *string           `yaml:"content-type"`
	FormatIndicator     *int              `yaml:"format-indicator"`
	Metadata            map[string]string `yaml:"metadata"`
	CorrelationIndex    *int              `yaml:"correlation-index"`
	CorrelationID       *string           `yaml:"correlation-id"`
	Qos                 *int              `yaml:"qos"`
	MessageExpiry       *TestCaseDuration `yaml:"message-expiry"`
	ResponseTopic       *string           `yaml:"response-topic"`
	InvokerIndex        *int              `yaml:"invoker-index"`
	PacketIndex         *int              `yaml:"packet-index"`
}

type TestCaseActionReceiveRequest struct {
	testCaseActionReceiveRequest
}

func (receiveRequest *TestCaseActionReceiveRequest) UnmarshalYAML(
	node *yaml.Node,
) error {
	*receiveRequest = TestCaseActionReceiveRequest{}

	receiveRequest.Topic = TestCaseDefaultInfo.Actions.ReceiveRequest.GetTopic()
	receiveRequest.Payload = TestCaseDefaultInfo.Actions.ReceiveRequest.GetPayload()
	receiveRequest.ContentType = TestCaseDefaultInfo.Actions.ReceiveRequest.GetContentType()
	receiveRequest.FormatIndicator = TestCaseDefaultInfo.Actions.ReceiveRequest.GetFormatIndicator()
	receiveRequest.CorrelationIndex = TestCaseDefaultInfo.Actions.ReceiveRequest.GetCorrelationIndex()
	receiveRequest.Qos = TestCaseDefaultInfo.Actions.ReceiveRequest.GetQos()
	receiveRequest.ResponseTopic = TestCaseDefaultInfo.Actions.ReceiveRequest.GetResponseTopic()
	receiveRequest.InvokerIndex = TestCaseDefaultInfo.Actions.ReceiveRequest.GetInvokerIndex()

	defaultMessageExpiry := TestCaseDefaultInfo.Actions.ReceiveRequest.GetMessageExpiry()
	receiveRequest.MessageExpiry = &defaultMessageExpiry

	return node.Decode(&receiveRequest.testCaseActionReceiveRequest)
}
