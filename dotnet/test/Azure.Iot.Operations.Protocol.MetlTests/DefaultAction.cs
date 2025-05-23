// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

namespace Azure.Iot.Operations.Protocol.MetlTests
{
    public class DefaultAction
    {
        public DefaultAction()
        {
            InvokeCommand = new();
            SendTelemetry = new();
            ReceiveRequest = new();
            ReceiveResponse = new();
            ReceiveTelemetry = new();
        }

        public DefaultInvokeCommand InvokeCommand { get; set; }

        public DefaultSendTelemetry SendTelemetry { get; set; }

        public DefaultReceiveRequest ReceiveRequest { get; set; }

        public DefaultReceiveResponse ReceiveResponse { get; set; }

        public DefaultReceiveTelemetry ReceiveTelemetry { get; set; }
    }
}
