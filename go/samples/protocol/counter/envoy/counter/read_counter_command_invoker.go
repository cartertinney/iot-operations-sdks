// Code generated by Azure.Iot.Operations.ProtocolCompiler v0.9.0.0; DO NOT EDIT.
package counter

import (
	"context"

	"github.com/Azure/iot-operations-sdks/go/protocol"
)

type ReadCounterCommandInvoker struct {
	*protocol.CommandInvoker[any, ReadCounterResponsePayload]
}

func NewReadCounterCommandInvoker(
	app *protocol.Application,
	client protocol.MqttClient,
	requestTopic string,
	opt ...protocol.CommandInvokerOption,
) (*ReadCounterCommandInvoker, error) {
	var err error
	invoker := &ReadCounterCommandInvoker{}

	var opts protocol.CommandInvokerOptions
	opts.Apply(
		opt,
		protocol.WithTopicTokens{
			"commandName":     "readCounter",
		},
	)

	invoker.CommandInvoker, err = protocol.NewCommandInvoker(
		app,
		client,
		protocol.Empty{},
		protocol.JSON[ReadCounterResponsePayload]{},
		requestTopic,
		&opts,
	)

	return invoker, err
}

func (invoker ReadCounterCommandInvoker) ReadCounter(
	ctx context.Context,
	executorId string,
	opt ...protocol.InvokeOption,
) (*protocol.CommandResponse[ReadCounterResponsePayload], error) {
	var opts protocol.InvokeOptions
	opts.Apply(
		opt,
		protocol.WithTopicTokens{
			"executorId": executorId,
		},
	)

	response, err := invoker.Invoke(
		ctx,
		nil,
		&opts,
	)

	return response, err
}
