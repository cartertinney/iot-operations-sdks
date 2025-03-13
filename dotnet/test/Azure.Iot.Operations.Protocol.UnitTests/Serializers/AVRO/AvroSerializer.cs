﻿// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

/* This file will be copied into the folder for generated code. */

namespace Azure.Iot.Operations.Protocol.UnitTests.Serializers.AVRO
{
    using System;
    using System.Buffers;
    using System.IO;
    using Avro;
    using Avro.IO;
    using Avro.Specific;
    using Azure.Iot.Operations.Protocol;
    using Azure.Iot.Operations.Protocol.Models;

    public class AvroSerializer<T1, T2> : IPayloadSerializer
        where T1 : class, ISpecificRecord, new()
        where T2 : class, ISpecificRecord, new()
    {
        private readonly SpecificDatumReader<T1> datumReader1;
        private readonly SpecificDatumWriter<T1> datumWriter1;

        private readonly SpecificDatumReader<T2> datumReader2;
        private readonly SpecificDatumWriter<T2> datumWriter2;

        public AvroSerializer()
        {
            Schema schema1 = new T1().Schema;
            datumReader1 = new SpecificDatumReader<T1>(schema1, schema1);
            datumWriter1 = new SpecificDatumWriter<T1>(schema1);

            Schema schema2 = new T2().Schema;
            datumReader2 = new SpecificDatumReader<T2>(schema2, schema2);
            datumWriter2 = new SpecificDatumWriter<T2>(schema2);
        }

        public const string ContentType = "application/avro";

        public const MqttPayloadFormatIndicator PayloadFormatIndicator = MqttPayloadFormatIndicator.Unspecified;

        public T FromBytes<T>(ReadOnlySequence<byte> payload, string? contentType, MqttPayloadFormatIndicator payloadFormatIndicator)
            where T : class
        {
            if (contentType != null && contentType != ContentType)
            {
                throw new AkriMqttException($"Content type {contentType} is not supported by this implementation; only {ContentType} is accepted.")
                {
                    Kind = AkriMqttErrorKind.HeaderInvalid,
                    HeaderName = "Content Type",
                    HeaderValue = contentType,
                    IsShallow = false,
                    IsRemote = false,
                };
            }

            try
            {
                if (payload.IsEmpty)
                {
                    if (typeof(T) != typeof(EmptyAvro))
                    {
                        throw AkriMqttException.GetPayloadInvalidException();
                    }

                    return (new EmptyAvro() as T)!;
                }

                using (var stream = new MemoryStream(payload.ToArray()))
                {
                    var avroDecoder = new BinaryDecoder(stream);

                    if (typeof(T) == typeof(T1))
                    {
                        T1 obj1 = new();
                        datumReader1.Read(obj1, avroDecoder);
                        return (obj1 as T)!;
                    }
                    else if (typeof(T) == typeof(T2))
                    {
                        T2 obj2 = new();
                        datumReader2.Read(obj2, avroDecoder);
                        return (obj2 as T)!;
                    }
                    else
                    {
                        return default!;
                    }
                }
            }
            catch (Exception)
            {
                throw AkriMqttException.GetPayloadInvalidException();
            }
        }

        public SerializedPayloadContext ToBytes<T>(T? payload)
            where T : class
        {
            try
            {
                if (typeof(T) == typeof(EmptyAvro))
                {
                    return new(ReadOnlySequence<byte>.Empty, null, 0);
                }

                using (var stream = new MemoryStream())
                {
                    var avroEncoder = new BinaryEncoder(stream);

                    if (payload is T1 payload1)
                    {
                        datumWriter1.Write(payload1, avroEncoder);
                    }
                    else if (payload is T2 payload2)
                    {
                        datumWriter2.Write(payload2, avroEncoder);
                    }
                    else
                    {
                        return new(ReadOnlySequence<byte>.Empty, null, 0);
                    }

                    stream.Flush();

                    byte[] buffer = new byte[stream.Length];
                    stream.Seek(0, SeekOrigin.Begin);
                    stream.Read(buffer, 0, (int)stream.Length);

                    return new(new(buffer), ContentType, PayloadFormatIndicator);
                }
            }
            catch (Exception)
            {
                throw AkriMqttException.GetPayloadInvalidException();
            }
        }
    }
}
