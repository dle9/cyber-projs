# image to use
FROM ubuntu:latest

# copy installation scripts into the container
COPY install/* /install/

# install packages
RUN /install/util.sh \
    ### java
    #&& /install/java.sh 8 \ 
    #&& /install/maven.sh 3.9.7 \
    #&& /install/gradle.sh 8.7 \ 
    # scala, sbt
    #&& /install/scala.sh 3.4.1 1.9.9

    ### node, npm
    #&& /install/node.sh 21.x \
    # yarn, bower 
    #&& /install/npm-packages.sh 1.22.10 1.8.10 \ 

    ### python, pip, pipenv
    #&& /install/python.sh \ 
    # poetry
    #&& /install/pip-packages.sh \ 

    ### ruby, rbenv, ruby-build
    #&& /install/ruby.sh 3.1.2  

    ### go
    #&& /install/go.sh 

    ### php
    && /install/php.sh \
    # composer, php plugins?
    && /install/php-plugins.sh

# test installations
RUN /install/check.sh

# cleanup

