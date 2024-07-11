#!/bin/bash

# setup
apt-get update -y \
&& apt-get install -y sudo

# create group and user
groupadd ${APPSEC_GROUP}
useradd -m -d ${APPSEC_USER_HOME} -g ${APPSEC_GROUP} -s /bin/bash ${APPSEC_USER}

# configure for sudo, use No Password
echo "${APPSEC_USER} ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/${APPSEC_USER}
chmod 0440 /etc/sudoers.d/${APPSEC_USER}

# misc setup
echo "export LANG=en_US.UTF-8" >> ${APPSEC_USER_HOME}/.profile
mkdir ${APPSEC_USER_HOME}/tmp