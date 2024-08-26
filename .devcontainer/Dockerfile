FROM mcr.microsoft.com/devcontainers/universal:2-linux

# Install dependencies, mosquitto client
RUN apt-get update && \
    apt-get install -y wget ca-certificates mosquitto-clients && \
    apt-get autoremove -y && \
    apt-get clean -y

# Install k3d
RUN wget --no-verbose -O /usr/local/bin/k3d https://github.com/k3d-io/k3d/releases/latest/download/k3d-linux-amd64 && \
    chmod +x /usr/local/bin/k3d

# Install Step CLI
RUN wget https://dl.smallstep.com/cli/docs-cli-install/latest/step-cli_amd64.deb && \
    sudo dpkg -i step-cli_amd64.deb && \
    rm step-cli_amd64.deb

# Install Dapr CLI
RUN wget -q https://raw.githubusercontent.com/dapr/cli/master/install/install.sh -O - | /bin/bash

# Install k9s
RUN wget https://github.com/derailed/k9s/releases/latest/download/k9s_linux_amd64.deb && \
    sudo dpkg -i k9s_linux_amd64.deb && \
    rm k9s_linux_amd64.deb

# Install mqttui
RUN wget https://github.com/EdJoPaTo/mqttui/releases/download/v0.21.0/mqttui-v0.21.0-x86_64-unknown-linux-gnu.deb && \
    sudo dpkg -i mqttui-v0.21.0-x86_64-unknown-linux-gnu.deb && \
    rm mqttui-v0.21.0-x86_64-unknown-linux-gnu.deb
