# image to use
FROM ubuntu:latest

# setup environment
RUN mkdir /scripts
COPY scripts/* /scripts
ENV APPSEC_GROUP=appsec-group
ENV APPSEC_USER=appsec-user
ENV APPSEC_USER_HOME=/home/${APPSEC_USER}

# initialize environment
RUN /scripts/init.sh
USER ${APPSEC_USER}
WORKDIR ${APPSEC_USER_HOME}

# build the container
RUN /scripts/install-utilities.sh
    ### java
    # && /scripts/install-java.sh 8 \ 
    # && /scripts/install-maven.sh 3.8.8 \
    # && /scripts/install-gradle.sh 7.6.4 \ 
    # && /scripts/install-scala-stuff.sh 3.3.3 1.9.0 \

    # ### node, npm
    # && /scripts/install-node.sh 21.x \
    # # yarn, bower, <more npm stuff here>
    # && /scripts/install-npm-packages.sh 1.21.1 1.8.14 \ 

    # ### python, pip, pipenv
    # && /scripts/install-python.sh \
    # # poetry <more pip packages here>
    # && /scripts/install-pip-packages.sh \ 

    # ### ruby, rbenv, ruby-build
    # #&& /scripts/install-ruby.sh 3.3.3 \
    # # cocoapods, <more ruby stuff here>
    # #&& /scripts/install-ruby-packages.sh 1.14.2 \

    # ### go
    # && /scripts/install-go.sh 1.21.5 \

    # ### php
    # && /scripts/install-php.sh 8.2 \
    # # composer, <more PHP plugins here>
    # && /scripts/install-php-stuff.sh 2.6.6 \

    # ### r
    # && /scripts/install-r.sh 4.3.3 \
    # # packrat, <more R packages here>
    # && /scripts/install-r-packages.sh 0.9.2 \

    # ### dotnet
    # && /scripts/install-dotnet.sh 8.0 \
    # # paket, <more dotnet stuff here>
    # && /scripts/install-dotnet-stuff.sh 8.0.0 \

    ### cargo
    # && /scripts/install-cargo.sh 1.70.0

# show versions of installations
RUN /scripts/check.sh

# cleanup

