// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

namespace Azure.Iot.Operations.Protocol.MetlTests
{
    public class TestCaseCatch
    {
        public const string HeaderNameKey = "header-name";
        public const string HeaderValueKey = "header-value";
        public const string TimeoutNameKey = "timeout-name";
        public const string TimeoutValueKey = "timeout-value";
        public const string PropertyNameKey = "property-name";
        public const string PropertyValueKey = "property-value";
        public const string CommandNameKey = "command-name";
        public const string RequestProtocolKey = "request-protocol";
        public const string SupportedMajorProtocolVersions = "supported-protocols";

        public string? ErrorKind { get; set; }

        public bool? IsShallow { get; set; }

        public bool? IsRemote { get; set; }

        public object StatusCode { get; set; } = false;

        public string? Message { get; set; }

        public Dictionary<string, string?> Supplemental { get; set; } = new();

        public AkriMqttErrorKind GetErrorKind()
        {
            return ErrorKind switch
            {
                "missing header" => AkriMqttErrorKind.HeaderMissing,
                "invalid header" => AkriMqttErrorKind.HeaderInvalid,
                "invalid payload" => AkriMqttErrorKind.PayloadInvalid,
                "timeout" => AkriMqttErrorKind.Timeout,
                "cancellation" => AkriMqttErrorKind.Cancellation,
                "invalid configuration" => AkriMqttErrorKind.ConfigurationInvalid,
                "invalid state" => AkriMqttErrorKind.StateInvalid,
                "internal logic error" => AkriMqttErrorKind.InternalLogicError,
                "unknown error" => AkriMqttErrorKind.UnknownError,
                "execution error" => AkriMqttErrorKind.ExecutionException,
                "mqtt error" => AkriMqttErrorKind.MqttError,
                "unsupported version" => AkriMqttErrorKind.UnsupportedVersion,
                _ => throw new Exception($"unrecognized error kind string \"{ErrorKind}\""),
            };
        }
    }
}
