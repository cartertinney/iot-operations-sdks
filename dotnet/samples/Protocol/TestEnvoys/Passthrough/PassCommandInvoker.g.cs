/* Code generated by Azure.Iot.Operations.ProtocolCompiler v0.10.0.0; DO NOT EDIT. */

#nullable enable

namespace TestEnvoys.Passthrough
{
    using System;
    using System.Collections.Generic;
    using Azure.Iot.Operations.Protocol;
    using Azure.Iot.Operations.Protocol.RPC;
    using Azure.Iot.Operations.Protocol.Models;
    using TestEnvoys;

    public static partial class Passthrough
    {
        /// <summary>
        /// Specializes the <c>CommandInvoker</c> class for Command 'pass'.
        /// </summary>
        public class PassCommandInvoker : CommandInvoker<byte[], byte[]>
        {
            /// <summary>
            /// Initializes a new instance of the <see cref="PassCommandInvoker"/> class.
            /// </summary>
            public PassCommandInvoker(ApplicationContext applicationContext, IMqttPubSubClient mqttClient)
                : base(applicationContext, mqttClient, "pass", new PassthroughSerializer())
            {
                this.ResponseTopicPrefix = "clients/{invokerClientId}"; // default value, can be overwritten by user code

                TopicTokenMap["modelId"] = "dtmi:com:example:Passthrough;1";
                if (mqttClient.ClientId != null)
                {
                    TopicTokenMap["invokerClientId"] = mqttClient.ClientId;
                }
                TopicTokenMap["commandName"] = "pass";
            }
        }
    }
}
