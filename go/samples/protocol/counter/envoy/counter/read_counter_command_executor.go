// Code generated by Azure.Iot.Operations.ProtocolCompiler v0.8.0.0; DO NOT EDIT.
package counter

import (
	"github.com/Azure/iot-operations-sdks/go/protocol"
)

type ReadCounterCommandExecutor struct {
	*protocol.CommandExecutor[any, ReadCounterResponsePayload]
}

func NewReadCounterCommandExecutor(
	app *protocol.Application,
	client protocol.MqttClient,
	requestTopic string,
	handler protocol.CommandHandler[any, ReadCounterResponsePayload],
	opt ...protocol.CommandExecutorOption,
) (*ReadCounterCommandExecutor, error) {
	var err error
	executor := &ReadCounterCommandExecutor{}

	var opts protocol.CommandExecutorOptions
	opts.Apply(
		opt,
		protocol.WithTopicTokens{
			"commandName": "readCounter",
		},
		protocol.WithIdempotent(false),
	)

	executor.CommandExecutor, err = protocol.NewCommandExecutor(
		app,
		client,
		protocol.Empty{},
		protocol.JSON[ReadCounterResponsePayload]{},
		requestTopic,
		handler,
		&opts,
	)

	return executor, err
}
