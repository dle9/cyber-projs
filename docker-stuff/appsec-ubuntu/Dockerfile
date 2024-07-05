# image to use
FROM ubuntu:latest

# copy installation scripts into the container
COPY install/* /install/

# install packages
RUN /install/util.sh \
    ### java
    && /install/java.sh 8 \ 
    && /install/maven.sh 3.8.8 \
    && /install/gradle.sh 7.6.4 \ 
    && /install/scala-stuff.sh 3.3.3 1.9.0 \

    ### node, npm
    && /install/node.sh 21.x \
    # yarn, bower, <more npm stuff here>
    && /install/npm-packages.sh 1.21.1 1.8.14 \ 

    ### python, pip, pipenv
    && /install/python.sh \
    # poetry <more pip packages here>
    && /install/pip-packages.sh \ 

    ### ruby, rbenv, ruby-build
    #&& /install/ruby.sh 3.3.3 \
    # cocoapods, <more ruby stuff here>
    #&& /install/ruby-packages.sh 1.14.2 \

    ### go
    && /install/go.sh 1.21.5 \

    ### php
    && /install/php.sh 8.2 \
    # composer, <more PHP plugins here>
    && /install/php-stuff.sh 2.6.6 \

    ### r
    && /install/r.sh 4.3.3 \
    # packrat, <more R packages here>
    && /install/r-packages.sh 0.9.2 \

    ### dotnet
    && /install/dotnet.sh 8.0 \
    # paket, <more dotnet stuff here>
    && /install/dotnet-stuff.sh 8.0.0 \

    ### cargo
    && /install/cargo.sh 1.70.0

# show versions of installations
RUN /install/check.sh

# cleanup

