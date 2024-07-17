#!/bin/bash

echo "export PATH=$PATH:\
$APPSEC_TOOL_DIR/go/bin:\
$APPSEC_TOOL_DIR/gradle/bin:\
$APPSEC_TOOL_DIR/maven/bin:\
$APPSEC_TOOL_DIR/coursier/bin:\
$HOME/.rbenv/versions/3.1.6/bin:\
$HOME/.rbenv/bin:\
$HOME/.dotnet/tools:\
$HOME/.local/bin:\
$HOME/.cargo/bin" >> $HOME/.bashrc
# JAVA_HOME=/usr/lib/jvm/java-8-openjdk-amd64/
# MAVEN_HOME=$APPSEC_TOOL_DIR/maven
# MAVEN_CONFIG=$HOME/.m2
# anything else not listed here is in its the default installation directory

echo "Installed package versions:"
exec /bin/bash
echo "java: $(java -version 2>&1 | awk -F '"' '/version/ {print $2}')"
echo "maven: $(mvn -version 2>&1 | awk '/Apache Maven/ {print $3}')"
echo "node: $(node -v)"
echo "npm: $(npm -v)"
echo "yarn: $(yarn -v)"
echo "bower: $(bower -v 2>&1)"
echo "gradle: $(gradle -v 2>&1 | awk '/Gradle/ {print $2}')"
echo "python3.9: $(python3.9 --version 2>&1 | awk '{print $2}')"
echo "pip3.9: $(pip3.9 --version | awk '{print $2}')"
echo "pipenv: $(pipenv --version 2>&1)"
echo "poetry: $(poetry --version 2>&1 | awk '{print $3}' | sed 's/.$//')"
echo "ruby: $(ruby -v | awk '{print $2}')"
echo "rbenv: $(rbenv -v | awk '{print $2}')"
echo "cocoapods: $(pod --version)"
echo "go: $(go version | awk '{print $3}')"
echo "scala: $(scala -version 2>&1 | awk -F ' ' '/version/ {print $5}')"
echo "sbt: $(sbt --script-version 2>&1)"
echo "php: $(php -v | head -n 1 | awk '{print $2}')"
echo "composer: $(composer --version | awk '{print $3}')"
echo "R: $(R --version | head -n 1 | awk '{print $3}')"
echo "packrat: $(R -e "packageVersion('packrat')" | grep -o '[0-9]*\.[0-9]*\.[0-9]*' | sed -n '2p')" 
echo "dotnet-sdk: $(dotnet --version)"
echo "paket: $(paket --version | awk '{print $3}')"
echo "cargo: $(cargo --version | awk '{print $2}')"

# cleanup
rm -rf $HOME/tmp \
&& rmdir $HOME/tmp \
&& sudo mv /scripts/check_versions.sh $HOME/ \
&& sudo rm -rf /scripts \
&& sudo rmdir /scripts \
&& sudo dnf autoremove \
&& sudo dnf clean all