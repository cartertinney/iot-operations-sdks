# Azure IoT Operation SDKs

This repository is for active development of the Azure IoT Operations SDKs. Visit Microsoft Learn for more information and [developing edge applications](https://learn.microsoft.com/en-us/azure/iot-operations/create-edge-apps/edge-apps-overview) or other components of [Azure IoT Operations](https://learn.microsoft.com/en-us/azure/iot-operations/).

There are three SDKs available, one for each language; .NET, Go and Rust.

## SDK components

This repository contains the following components:

| Component | Description | [Go](./go) | [.NET](./dotnet) | [Rust](./rust) |
|-|-|-|-|-|
| Session client | Creates the underlying MQTT client, authenticates against MQTT Broker and maintains the connection. | :green_circle: | :green_circle: | :green_circle: |
| State store client | Client that enables interaction with the state store. | :green_circle: | :yellow_circle: | :yellow_circle: |
| Lease lock client | Create a lock for a shared resource | :green_circle: | :yellow_circle: | :yellow_circle: |
| Leader election client | | :green_circle: | :yellow_circle: | :yellow_circle: |
| Schema registry client | | :green_circle: | :yellow_circle: | :yellow_circle: |
| Protocol compiler | | :green_circle: | :yellow_circle: | :yellow_circle: |
||||||

## Getting started

## Packages available

## Samples

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft 
trademarks or logos is subject to and must follow 
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.
