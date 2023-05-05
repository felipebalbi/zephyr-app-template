# Developer Image (zephyr-build)

ARG BASE_IMAGE
FROM ${BASE_IMAGE:-zephyrprojectrtos/ci:latest}

# Install packages
RUN apt-get -y update && \
	apt-get -y upgrade && \
	apt-get install --no-install-recommends -y \
	python3-xdg \
	libpython3-dev \
	xz-utils

# Clean up stale packages
RUN apt-get clean -y && \
	apt-get autoremove --purge -y && \
	rm -rf /var/lib/apt/lists/*

# Switch to 'user' context
USER user

# Configure environment variables
ENV ZEPHYR_BASE=/workspaces/zephyr-app-template/zephyr

# Set working directory
WORKDIR /workspaces
VOLUME ["/workspaces"]

# Adjust 'user' home directory permissions
USER root
RUN chown -R user:user /home/user

# Make 'user' default on launch
USER user

ENTRYPOINT ["bash"]
