// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
package resp_test

import (
	"testing"

	"github.com/Azure/iot-operations-sdks/go/services/statestore/internal/resp"
	"github.com/stretchr/testify/require"
)

func TestFormatBlobArray(t *testing.T) {
	require.Equal(t,
		[]byte("*3\r\n$3\r\nSET\r\n$7\r\nSETKEY2\r\n$6\r\nVALUE5\r\n"),
		resp.OpKV("SET", "SETKEY2", "VALUE5"),
	)
	require.Equal(t,
		[]byte("*2\r\n$3\r\nGET\r\n$7\r\nSETKEY2\r\n"),
		resp.OpK("GET", "SETKEY2"),
	)
	require.Equal(t,
		[]byte("*2\r\n$3\r\nDEL\r\n$7\r\nSETKEY2\r\n"),
		resp.OpK("DEL", "SETKEY2"),
	)
	require.Equal(t,
		[]byte("*3\r\n$4\r\nVDEL\r\n$7\r\nSETKEY2\r\n$3\r\nABC\r\n"),
		resp.OpKV("VDEL", "SETKEY2", "ABC"),
	)
}
