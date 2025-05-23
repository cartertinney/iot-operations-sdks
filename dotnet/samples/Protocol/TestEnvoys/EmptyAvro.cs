// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

/* Code generated by Azure.Iot.Operations.ProtocolCompiler v0.10.0.0; DO NOT EDIT. */

namespace TestEnvoys
{
    using Avro;
    using Avro.Specific;

    public class EmptyAvro : ISpecificRecord
    {
        public Schema Schema
        {
            get => PrimitiveSchema.Create(Schema.Type.Null);
        }

        public object Get(int fieldPos)
        {
            return null!;
        }

        public void Put(int fieldPos, object fieldValue)
        {
        }
    }
}
