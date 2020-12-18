#!/bin/bash

echo "Installing Universal Discovery into /var/db/newrelic-infra/"

if [ `whoami` != root ]; then
    echo Please run the installer as root or using sudo
    exit 1
fi

# remove any previous versions
rm -rf  /var/db/newrelic-infra/nri-universal-discovery

# create dir path if it doesn't exist
mkdir -p /var/db/newrelic-infra/

# move binary
mv nri-universal-discovery /var/db/newrelic-infra/

# create a separate dir for universal-discovery related configs
mkdir -p /etc/newrelic-infra/integrations.d/universal-discovery-sub-configs/