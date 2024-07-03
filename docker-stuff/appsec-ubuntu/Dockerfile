# image to use
FROM ubuntu:latest

# copy installation scripts into the container
COPY install/* /install/

# install packages
RUN /install/util.sh \
    ### java stuff
    && /install/java.sh 8 \ 
    #&& /install/maven.sh 3.9.7 \
    #&& /install/gradle.sh 8.7 \ 
    # scala, sbt
    && /install/scala.sh 3.4.1 1.9.9

    ### node, npm stuff
    #&& /install/node.sh 21.x \
    # yarn, bower 
    #&& /install/npm-packages.sh 1.22.10 1.8.10 \ 

    ### python, pip, pipenv stuff
    #&& /install/python.sh \ 
    # poetry
    #&& /install/py-packages.sh \ 

    ### ruby, rbenv, ruby-build
    #&& /install/ruby.sh 3.1.2  

    ### go
    #&& /install/go.sh 


# test installations
RUN /install/check.sh

# cleanup

