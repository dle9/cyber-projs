# A Guide to Making Your Own Virtual Homelab
---
July 29, 2023    |    Created by: Deric Le

*Note: always check the hash of downloaded files to verify its integrity! Just search "check hash of file" on Youtube for additional information*
*Hardware requirements: opnsense(2000mb, 8gb)*

### Scope
---

![[Pasted image 20230729131546.png]]

Things you will learn:
###### General:
- VMs: Metasploitable, Arch Linux VM installation, setup, configuration
- Software: Burpsuite, Zed Attack Proxy (ZAP), DirBuster, Nessus 
- Linux tools/commands: nmap, pgrep, curl, hping, scp, wafw00f, nikto
###### Firewall:
- OPNsense installation/configurations
- Creating and testing Suricata rules to detect malicious activity  
###### Web server:
- Apache/MySQL/OpenSSH installation/configuration
- Creating and testing Apache websites
- Simple Apache and MySQL commands
- Apache WAF module (ModSecurity) installation and configuration
- Vulnerability scanning, SQL injection, brute forcing, proxying, XSS

### How do we start? 
---
It would be a pain to buy all of the required hardware... let's build a virtual one! First, we'll need to plan our virtual network. We can use [Smartdraw](https://smartdraw.com) to visualize it. 
$$\textit{P.S. holding 'ctrl' while clicking on a link will open it in a new tab}$$

![[Pasted image 20230729133657.png]]

Alright, we got some basic stuff down. We'll need some servers as well, but just leave it blank for now. All you need to know is that servers provide services to clients (e.g., PC1, PC2) within a network. For example, an email server is responsible for sending, receiving, and storing emails. Simple, right? 

![[Pasted image 20230729134758.png]]

Oh! We almost forgot the firewall. This is an essential network component since we will be exposed to the internet. It will help to protect our internal network (what we have so far) from unauthorized access.

![[Pasted image 20230729134917.png]]

There we go, our network diagram is finished. Once it's implemented, you can test it using a virtual machine of your choice to check that it is working correctly and securely. This will be referenced as your attack VM. A good option would be to use [Kali Linux](https://www.kali.org/get-kali/#kali-installer-images), which comes preinstalled with cybersecurity tools. If you're looking for a more lightweight distribution, you can use [Arch Linux](https://www.youtube.com/watch?v=uAf6gjPgkPo) (this is what I'll be using; you'll have to manually install any required tools). 
$\quad$
$\quad$
$\quad$$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$

### Firewall: Setting up OPNsense
---
For our firewall, we'll be using [OPNsense](https://opnsense.org/download/) (Version 23.1), a FreeBSD-based firewall. Navigate to the OPNsense website and select "dvd" under image type and select a mirror closest to your current location. Once it's downloaded, you can verify its integrity using the provided check sum (shown at the bottom of the OPNsense downloads screen):

![[Pasted image 20230729142730.png]]
![[Pasted image 20230729142927.png]]

$\quad$
$\quad$

Alright, it matches! Remember, this is good practice for any file you download off of the internet. Now you can unzip the file ([WinRAR](https://www.win-rar.com/download.html?&L=0)) and use your virtualization software of choice. I'm using [VirtualBox](https://www.virtualbox.org/) (Version 7.0.8). Click on 'New' and enter the following:

![[Pasted image 20230730151904.png]]

$$\text{Name: \textit{name of your choice}}$$
$$\text{Folder: \textit{where you want it to be stored}}$$
$$\text{ISO image: \textit{location of the decompressed OPNsense .iso file}}$$
$$\text{Type: BSD}$$
$$\text{Version: FreeBSD (32/64-Bit)} \leftarrow \textit{this depends on your OS specifications}$$
$$\text{Base Memory: At least 2000 MB}$$
$$\text{Processors: At least 1}$$
$$\text{Create a virtual hard disk now: } \textit{Recommended 8 GB}$$
$\quad$
Once that's done, click 'Finish.' Next, we need to configure the network adapters of our firewall. 

$\quad$
$\quad$
$\quad$
![[Pasted image 20230729145034.png]]

Let's see how we can translate this into our virtual environment. Open up the network settings in OPNsense:

$$\text{Settings > Network}$$

![[Pasted image 20230730152030.png]]

Next, configure the firewall's network like so:
$$\text{Network adapter 1: NAT}$$
$$\text{Network adapter 2: Internal network}$$

The default network adapter is "NAT." Using this, we can connect to the internet. Therefore, we have our WAN adapter. Next, we add "Internal Network" on Network adapter 2. This adapter provides a private, isolated environment for communication between our virtual machines; our LAN.

Finally, we can start our firewall. Double-click it to start the initial installation. You should see text flying across the screen, like this:

![[Pasted image 20230729151015.png]]

Since it is our first time setting up the firewall, enter this once you reach the login prompt:

$$
\text{login: installer} $$
$$
\text{password: opnsense}$$
![[Pasted image 20230730152150.png]]

Confirm all default settings besides the following:

$$
-\text{UFS Configuration, Please select a disk to continue: ada0}$$
$$-\text{Root password:} \textit{ choose a password}$$
$$-\text{Complete install, Reboot}$$

While rebooting, take a look at the VirtualBox window. We won't be needing the OPNsense disk anymore since we just installed it. Click:
$$\text{Devices > Optical Drives > Remove disk from optical drive}$$


![[Pasted image 20230730152426.png]]

After a while, you should see the login prompt. If you don't, reset the VM `(Host+r)`.
$$\textit{P.S. The Host key in VirtualBox is by default 'right-ctrl'}$$
Enter your credentials:
$$\text{login: root}$$
$$\text{password: \textit{password you chose}}$$

Next, we should configure the network adapters we set up earlier:
$$\text{Enter an option: 1}$$
$$\text{Configure LAGGs: No}$$
$$\text{Configure VLANs: No}\leftarrow\textit{this will be done later}$$
$$\text{WAN interface name: em0}\leftarrow(\textit{Network adapter 1, NAT})$$
$$\text{LAN interface name: em1}\leftarrow(\textit{Network adapter 2, Internal network})$$
$$\text{Optional interface 1 name: \textit{leave empty and proceed}}$$

To check if we did it correctly, we can use our attack VM with the same Network adapters as our firewall, that is,
$$\text{Network adapter 1: NAT}$$
$$\text{Network adapter 2: Internal network}$$
and ping our firewall IP address. Here is what you should see:

![[Pasted image 20230729165941.png]]

Once you have verified connectivity, open a web browser and login using the root credentials. Go through with the wizard, and leaving everything as default besides 

$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$

$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
### Firewall: Basic OPNsense Configurations
---

R-U-N? Great. Take a look at the Dashboard. What services are running by default? Do you recognize any of them? Try adding a new user at
$$\text{System > Access}$$
Check out network traffic at:

$$\text{Reporting > Traffic}$$
Try pinging the firewall again. Do you notice anything?

Alright, once you're done exploring, head over to 
$$\text{System > Firmware > Status}$$
Click on 'Check for updates.' This should take you to the 'Updates' tab, where you will confirm the update. 

![[Pasted image 20230730151309.png]]

After you have completed the update, head over to the 'Plugins' tab.

![[Pasted image 20230730153938.png]]

Since we are using OPNsense on a virtual machine, we'll want to install the 'os-virtualbox' plugin to make our virtual firewall more compatible/efficient with VirtualBox. 

Ok, now OPNsense should be performing at its best capabilities. Next, we'll need to install threat detection software (IDS/IPS).

$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
$\quad$
### Firewall: Intrusion Detection with Suricata
---

To put it simply, an intrusion detection system (IDS) finds network anomalies that might indicate malicious activity (malware, unauthorized access, cyberattacks, etc.). An intrusion prevention system (IPS) does the same, except instead of just generating alerts, it takes action to prevent the threat. 

IPS might seem like the better option most of the time, but as an exercise, take a second to think when an IDS would be preferred. For this lab, we'll be using Suricata, a widely used IDS/IPS software tool. Now let's set up Suricata. To start, head over to
$$\text{Services > Intrusion Detection > Administration}$$
and configure these settings like so:

![[Pasted image 20230731221842.png]]

Make sure that $\text{Home networks}$ is the LAN subnet that OPNsense is operating on. The rest you can leave as default. You can see why we made these decisions [](https://docs.opnsense.org/manual/ips.html#ids-and-ips). Click 'Apply,' and then the 'Download' tab. Here you can see the available rulesets. You can research and enable whichever rules you require, but for this project, we'll be writing our own [Suricata Rules](https://docs.suricata.io/en/latest/rules/index.html)

To do this, we'll need to enable SSH to access the OPNsense files and insert our new rule. Go to 

$$\text{System > Settings > Administration}$$

and change the following:

![[Pasted image 20230731115404.png]]

Don't forget to save! Ok, now create a $\text{custom\_suricata\_rules}$ directory anywhere you want and navigate into it.


![[Pasted image 20230731151205.png]]

$\quad$
$\quad$
$\quad$
$\quad$

We'll need to establish an SFTP session (SFTP is secured by SSH, so it will operate on port 22!)
$$\text{sftp root@\textit{OPNsense\_LAN\_IP}}$$
![[Pasted image 20230731150317.png]]

I'll save you the trouble of looking for the $\text{rules}$ directory. Head to 
$$\text{/usr/local/opnsense/scripts/suricata/metadata/rules}$$
![[Pasted image 20230731151310.png]]

Get one of the xml files so we can take a look.

![[Pasted image 20230731151343.png]]

Ok, it was successful. Open up a new terminal and check it out.

![[Pasted image 20230731151440.png]]

Wow, that's a lot of information. Let's keep this syntax in mind when we create our first rule for detecting nmap scans. Open up a text editor of your choice and create a file named $\text{detect\_nmap.rules}$, which will contain:

`alert tcp $HOME_NET any -> DEST_IP/24 any (msg:"Possible nmap SYN scan detected."; flow:stateless; flags:S; priority:5; threshold:type threshold, track by_src, count 50, seconds 1; classtype:attempted-recon; sid:1234;)`

More information [here](https://docs.suricata.io/en/latest/rules/intro.html). Enter your OPNsense LAN ip address for $\text{DEST\_IP}$, and save the file to the directory we just created. In the same directory, create another file named $\text{detect\_nmap.xml}$. This will tell OPNsense where to get the rules from. It will contain: 

`<?xml version:"1.0"?>`
`<ruleset documentation_url="https://docs.opnsense.org/">`
$\quad\quad\quad$`<location url="https://ATTACK_LAN_IP/" prefix="detect_nmap"/>`
$\quad\quad\quad$`<files>`
$\quad\quad\quad$$\quad\quad\quad$`<file description="detect_nmap">detect_nmap.rules</file>`
$\quad\quad\quad$$\quad\quad\quad$`<file description="detect_nmap" url="inline::rules/detect_nmap.rules">detect_nmap.rules</file>`
$\quad\quad\quad$`</files>`
`</ruleset>`

Replace $\text{'ATTACK\_LAN\_IP'}$. Go back to the sftp session terminal and put the file into the rules directory: 
$$\text{/usr/local/opnsense/scripts/suricata/metadata/rules}$$
![[Pasted image 20230731152247.png]]

Now OPNsense knows where to query for the '.rules' file. There is one last step, though.  You can close the sftp session. Finally, we need to set up an HTTP server that will respond to the OPNsense queries. 

![[Pasted image 20230731161432.png]]
$$\textit{(If the message 'Serving HTTP...' doesn't appear, try opening http.servers on}$$
$$\textit{443, 8000, 8080 and forcibly closing them. More information below.)}$$
Make sure that it's working by using `curl`.

![[Pasted image 20230731161145.png]]
![[Pasted image 20230731161128.png]]

If you're experiencing any issues, use `pgrep -f "python3 -m http.server"` to get the process ID (PID) and forcibly terminate it using `kill -9 PID`. If you're ready to move on, head back to the OPNsense web GUI and navigate here

![[Pasted image 20230731153326.png]]

You should see our new rule on this list. If you don't, restart services (button), refresh the page, or restart OPNsense. 

![[Pasted image 20230731162610.png]]

Check the box next to it, then click 'Enable selected.' You should now see that it is enabled.

![[Pasted image 20230731162721.png]]

Finally, scroll down and click 'Download & Update Rules.' You should see:

![[Pasted image 20230731162814.png]]

Double check that it is active:

![[Pasted image 20230731162905.png]]

You can also click on the 'Info' button to see more information about this rule.

![[Pasted image 20230731163002.png]]

Take a look at the 'Schedule' tab. We won't be getting into that in this project, but keep in mind that it is a very important tool for managing your rules.

Now that we have successfully implemented our SYN scan detection rule, we can test it out with `nmap`:
$$\text{nmap \textit{OPNsense\_LAN\_IP} -F}$$
$$\textit{-F specifies 'fast,' i.e. the 100 most common ports}$$
Replace $\text{OPNsense\_LAN\_IP}$.

![[Pasted image 20230731215523.png]]

You can see that `nmap` was able to determine any open ports within the 100 most common ports, and that our firewall is running on VirtualBox. Now, navigate to the 'Alerts' tab and refresh the page. 

![[Pasted image 20230801113545.png]]

It looks like our new rule was able to detect the nmap scan, as well as the source of it. This is because nmap sends TCP SYN packets by default, and we specified SYN packets in our rule (`flags:S`). 


Now, try sending UDP packets with `nmap`.
$$\text{sudo nmap OPNsense\_LAN\_IP -sU -F}$$

![[Pasted image 20230731220822.png]]

As you can see, no alerts were generated. Try the `ping` command as well! (spoiler; `ping` sends ICMP packets).

We should try sending TCP SYN packets with `hping`, a common tool for firewall/network testing.
$$\text{sudo hping3 OPNsense\_LAN\_IP -c 3 -S -p 80}$$

![[Pasted image 20230731230517.png]]

We successfully sent 3 TCP SYN packets, but our rule didn't generate any alerts. Why is that? To understand this, we need to know what exactly [SYN packets](https://www.geeksforgeeks.org/tcp-3-way-handshake-process/) are. 

When you only send SYN packets, you aren't establishing a complete connection with the target, making SYN scans faster than a full TCP handshake. In return, the target sends back a SYN-ACK packet, which when received, confirms that the target port is open. 

Ok, back to hping. Why aren't any alerts being generated? Take a closer look at our rule. 

`alert tcp $HOME_NET any -> DEST_IP/24 any (msg:"Possible nmap SYN scan detected."; flow:stateless; flags:S; priority:5; threshold:type threshold, track by_src, count 50, seconds 1; classtype:attempted-recon; sid:1234;)`

So, we know that the 'S' flag specifies SYN packets, and a 'stateless' flow inspects individual packets without considering previous packets. I don't think that's the issue. Take a look at the other arguments. `track by_src, count 50`, and `seconds 1` mean that if it matches an event 50 times within 1 second from the same IP address, it'll generate an alert. 

Now we're getting somewhere. Is `hping` slower than `nmap`? Reduce `count` to see what happens. Open up a terminal and navigate to your $\text{detect\_nmap.rules}$ file. Reduce `count` to 1.

![[Pasted image 20230801105122.png]]

There we go. Make sure to save and update any changes.

![[Pasted image 20230801104216.png]]

Huh... it didn't update. Oh! That's right! We need to set up the HTTP server again so that the suricata request can go through. 

![[Pasted image 20230801113954.png]]
![[Pasted image 20230801104613.png]]

Alright it successfully updated. Let's check if this still works with `nmap`.

![[Pasted image 20230801110506.png]]

It generated way more alerts! Remember that `count 50` only generated 3 alerts. Now try `hping`. 

![[Pasted image 20230801112145.png]]

We sent 3 SYN packets using `hping`, and 3 alerts were generated; We can conclude that our original rule is not tailored specifically to the `nmap` scan, and only detects fast SYN scans. We should also set `count` to 100 to see if `nmap` scans are still detected.

Don't forget to save the file, start an HTTP server, and update the rules. 

![[Pasted image 20230801112828.png]]

This time, only 1 alert was generated. Earlier, `nmap -F` generated 3 alerts with the `count 50` argument. It seems like `nmap -F` is able to send at least 150 alerts within a second. Finally, try `count 190`.

![[Pasted image 20230801113253.png]]

It looks like `nmap -F` isn't able to send 190 SYN packets within a second. Ok that's enough for now. Change `count` back to 50. 



### Web/file/database server
---
$$\textbf{\color{red}IMPORTANT: After many frusturating issues with Arch Linux,}$$
$$\textbf{\color{red}I have decided to use Ubuntu for Bernard. Following this decision,}$$
$$\textbf{\color{red}I switched to Kali Linux for my attack VM as wekk. There will be many changes}$$
$$\textbf{\color{red}to commands and file locations, so please use ChatGPT for quick translations.}$$
Bernard is the name I have given my web/file/database server. He is a simple arch linux machine with `apache`, `openssh`, and `mariadb` installed. 
$$\text{sudo pacman -S apache mariadb openssh}$$
$\textbf{\color{red}Guide for Ubuntu apache server}$ [here](https://ubuntu.com/tutorials/install-and-configure-apache\#1-overview). If you want to continue with Arch Linux, follow below.
$$\textbf{\color{red}Place web directories inside /var/www/html to access via http://IP/website}$$

Let's start with adding websites to bernard's apache web server. By default, this is located in 
$$\text{/srv/http}$$
In this directory, I have created $\text{custom-prompt}$. 

![[Pasted image 20230803210334.png]]

Throughout this project, you've probably been wondering: "How does your prompt look so cool?" Take a look through this custom site that I made!

![[Pasted image 20230803211058.png]]

`PS1` (Prompt String) is a variable in ~$\text{/.bashrc}$ defining the appearance of your prompt.

![[Pasted image 20230803211400.png]]

It should look something like that. Look! There's the `PS1` variable. Assign it to the custom prompt.

![[Pasted image 20230803211417.png]]

There. Reset your terminal. 
$$\textit{P.S. the pink text comes from the terminal 'Preferences' setting}$$

Alright, by definition, we have a web server. To test this, I'll be using the 'Bridged adapter'. This adapter simply allows your virtual machines to be on the same local network as your host machine. Let's test connectivity from the host machine.

![[Pasted image 20230803212849.png]]

Alright! Now, let's access it through another machine.

![[Pasted image 20230803213134.png]]

Now our host machine.

![[Pasted image 20230803213214.png]]

Perfect! We have confirmed that our web server is working. So, how did I do this? It's pretty simple actually. First, 
$$\text{sudo systemctl start httpd}$$
This one is self explanatory. Then, 
$$\text{sudo systemctl enable httpd}$$
This allows our web server to start on boot-up. Then, I added an $\text{index.html}$ file (if you don't know much about the file structure of web pages, see [here](https://developer.mozilla.org/en-US/docs/Learn/Getting_started_with_the_web/Dealing_with_files)). It contains:

![[Pasted image 20230803214001.png]]

And that's all there is to it. You create a directory, add an index.html file, and magically, a web page appears. Let's create another web page. We'll include an image this time. 

![[Pasted image 20230803214157.png]]

Booboo is one of my dogs! You'll get to see him soon. 

![[Pasted image 20230803214324.png]]

Now, find some images and upload them to bernard using [OpenSSH](https://wiki.archlinux.org/title/OpenSSH). To do this, we'll create a $\text{/srv/files}$ directory with the following permissions:

![[Pasted image 20230804115213.png]]

Now, after installing openssh and starting the server (`sudo systemctl start sshd`), we can use `scp` on our Windows host machine like so to transfer our files:

![[Pasted image 20230804115853.png]]
$$\textit{scp source\_path dest\_user@dest\_ip:/dest\_path}$$
Looks like there's a problem with file transfer using root login. Let's try bernard's login.

![[Pasted image 20230804120046.png]]

Nice! That's a bit too slow, though. Here's how  you can send multiple files at once:

![[Pasted image 20230804120335.png]]

Time to check if they were transferred correctly.

![[Pasted image 20230804120419.png]]

Bernard got the files! Add these to your webpage.

![[Pasted image 20230804120733.png]]
![[Pasted image 20230804121304.png]]
![[Pasted image 20230804121045.png]]
![[Pasted image 20230804121339.png]]

The images turned out a bit too big, but here he is!

![[Pasted image 20230804121519.png]]

Would you look at that! Our file server has been taken care of as well. So far, bernard consists of a web server (Apache: httpd) and a file server (OpenSSH: sshd, scp). Now, we just need a database server. We'll be using MariaDB, a fork of MySQL. 

Before starting, initialize `mariadb`
$$\text{mariadb-install-db --user=mysql --basedir=/usr --datadir=/var/lib/mysql}$$
Now, start and enable the `mariadb` service. You should now see all three of our server ports open on bernard.

![[Pasted image 20230804161013.png]]

Go through with the initial setup: `sudo mariadb-secure-installation`. After that, create a new user. 

$$\text{CREATE USER 'some\_usr'@'\%' IDENTIFIED BY 'some\_pass';}$$
Grant access on a subnet (`x.x.x.%`) or to any domain (`%`).  
$$\text{GRANT ALL PRIVILEGES ON *.* TO 'some\_usr'@'\%' WITH GRANT OPTION;}$$
Log in as your new user to see that it is working correctly.
$$\textbf{\color{red}Ubuntu: By default, you can only connect to the mysql server locally.}$$
$$\textbf{\color{red}Go to /etc/mysql/mysql.conf.d/mysqld.cnf and set bind-address=0.0.0.0}$$
![[Pasted image 20230804165346.png]]

Now, try to log in on another machine. I am using my Windows host machine's Ubuntu WSL (`sudo apt install mariadb-client`).

![[Pasted image 20230804170436.png]]

It works! Let's mess around a bit to refresh our SQL knowledge.

![[Pasted image 20230804170529.png]]

This reminds me of the Avatar universe (not the blue guys, I'm talking about the bald kid). Let's add some characters from Avatar as well as their bending art. 

![[Pasted image 20230804170747.png]]
![[Pasted image 20230804171109.png]]
![[Pasted image 20230804171928.png]]
![[Pasted image 20230804172544.png]]

Ok, now exit and open connect to `MariaDB` on bernard. 

![[Pasted image 20230804172626.png]]
![[Pasted image 20230804172720.png]]

Awesome! Our database works. Now bernard is ready to go. Finally, we can use our attack VM to test some vulnerabilities in bernard. 

### Zram Sidequest (ignore)
---
$$\textit{This section is not a part of this project. You can skip over it.}$$
This morning I woke up and started bernard. I got an error along this lines of: $$\text{Timed out waiting for /dev/zram0}$$
Pressing `Ctrl+Alt+F2` allows me to access the terminal on the bootup screen. Looking at the documentation, I can use `zramctl` to check the status of it. 

![[Pasted image 20230805110149.png]]

Ok, looks like we got a situation. Is it even enabled?

![[Pasted image 20230805110253.png]]

N as in no? Ok let's reverse it

![[Pasted image 20230805110610.png]]

![[Pasted image 20230805110739.png]]
![[Pasted image 20230805110758.png]]

Oh, guess I should've checked that first...I wonder why we don't have a zram file here. Let's check our attack VM.

![[Pasted image 20230805110940.png]]

![[Pasted image 20230805111603.png]]

That explains it. 

![[Pasted image 20230805111913.png]]

Zram=m means that we should have a zram kernel module. Since bernard uses a linux module, we can try to reinstall it. 
$$\text{sudo pacman -S linux}$$
![[Pasted image 20230805112449.png]]

(None of this worked)

![[Pasted image 20230809213220.png]]

After a couple hours, I've decided to just use Ubuntu. Goodbye.


### Metasploitable: Introducing Burpsuite Proxy
---
$$\textbf{\color{red}Note: I have added msf.local to my /etc/hosts file to map to this IP address}$$

DVWA, or Damn Vulnerable Web Application, was created specifically for web application security testing. Now that we know how to set up a web server and our own website, let's first take a look at how we can pen test a professionally created virtual web application.

To access DVWA, first install [Metasploitable](https://sourceforge.net/projects/metasploitable/) by following [this](https://www.geeksforgeeks.org/how-to-install-metasploitable-2-in-virtualbox/) guide to set it up in VirtualBox. After accessing the ip address of metasploitable from your attack VM web browser, you should see

![[Pasted image 20230806181014.png]]

Click $\text{DVWA}$ and login
$$\text{username=admin, password=password}$$
Change the security level to `low`, since we are just beginners

![[Pasted image 20230806181122.png]]

The first task we will perform is 'spidering.' This will give us a map/scope of the target website. To understand this better, first open up the $\text{Proxy}$ tab in [Burpsuite](https://www.geeksforgeeks.org/what-is-burp-suite/).

![[Pasted image 20230806194150.png]]

Here you can intercept and modify requests. Open up the $\text{Proxy settings}$. You can see the default proxy listening port and IP address.

![[Pasted image 20230806194345.png]]

You can either manually enable this in your browser's proxy settings, or just click $\text{Open browser}$ in Burpsuite.

![[Pasted image 20230806194509.png]]

After that, turn on $\text{Intercept}$. And send a test login request to the $\text{Brute Force}$ tab in DVWA.

![[Pasted image 20230806195009.png]]

Burpsuite intercepted the request! You can either *drop* it or *forward* it. Try pressing $\text{Forward}$

![[Pasted image 20230806195140.png]]

As you can see, the request only went through once we *forwarded* the intercepted request. Send another login request and take a closer look at the intercepted information.



### Metasploitable: Brute forcing with Burpsuite Intruder
---

After exploring the intercepted information, right click it and press $\text{Send to Intruder}$ (`Ctrl+I`). 

![[Pasted image 20230806195907.png]]

Then, forward the payload and head over to the $\text{Intruder}$ tab. 

![[Pasted image 20230806200441.png]]

You can see the modifiable positions highlighted in red. Next, highlight the username and password and add new markers there since that is all we need to brute force the login.

![[Pasted image 20230806200558.png]]

It should look like this:

![[Pasted image 20230806200647.png]]

Now, let's configure the attack. Change $\text{Attack type}$ to [Cluster Bomb](https://portswigger.net/burp/documentation/desktop/tools/intruder/configure-attack/attack-types#:~:text=Cluster%20bomb,all%20payload%20combinations%20are%20tested.) since we will be brute forcing the username *and* the password. Now switch over to the $\text{Payloads}$ tab.

![[Pasted image 20230806202008.png]]

Payload set: 1 refers to the username, and payload set: 2 refers to the password. For the username, load up a wordlist. Since I am using Kali, I have several wordlists in 
$$\text{/usr/share/metasploit-framework/data/wordlists}$$
I'll use $\text{namelist.txt}$

![[Pasted image 20230806202352.png]]

For the password, I'll use $\text{password.lst}$

![[Pasted image 20230806202442.png]]

Now, start the attack (this will take a while, if you want, you can manually enter in "admin" for the username list and "password" for the password list). You can see the number of possible combinations from these two lists here. 

![[Pasted image 20230806202732.png]]

Since I am using Burpsuite community edition, it is executing under one combination per second. This means that it would take at least $\text{168751782/60/60/24=1953 days}$. No thanks. Let's add our own word lists.

![[Pasted image 20230806203110.png]]
![[Pasted image 20230806203132.png]]

Take a look at the outputs. Notice anything different?

![[Pasted image 20230806203341.png]]

One of the lengths is 4984. This is the only unique length. Click on the unique username, password combination and view its response.

![[Pasted image 20230806203516.png]]


### Metasploitable: Site mapping with Burpsuite
---

In this section, we'll be learning about mapping out a website. This gives us an idea of the structure of our target website. To do this, open Burpsuite and enable manual proxy in your browser settings. Then, navigate to 
$$\text{Target > Site Map}$$
and reload your webpage. In this case, we'll be using Mutillidae found in our metasploitable VM.

![[Pasted image 20230807135341.png]]

Take a closer look at the discovered web site paths. Let's try one of them.

![[Pasted image 20230807143010.png]]


### Metasploitable: Site mapping & discovering hidden files with ZAP
---

[ZAP](https://www.zaproxy.org/download/) or $\text{Zed Attack Proxy}$ to discover hidden files. Make sure you have enabled your browser's proxy. After opening it up, select this 

![[Pasted image 20230807143246.png]]

By default, the proxy settings for ZAP are the same as Burpsuite, so you won't have to change any proxy settings in your browser (keep in mind that since they are the same, you can't run both at the same time, you'll have to change the port or IP address). Reload Mutillidae. You should see a new entry under $\text{Sites}$. Open it up and right click mutillidae to start a [Spider](https://en.wikipedia.org/wiki/Web_crawler) scan.

![[Pasted image 20230807144119.png]]

After that has finished, 

![[Pasted image 20230807144620.png]]

Right click the $\text{mutillidae}$ folder again and start the following attack

![[Pasted image 20230807144656.png]]

This will open up a new tab. In this tab, select the default word list and right click the mutillidae folder again to start the attack (don't press the play button, as this will not focus on mutillidae).

Now, ZAP will execute a [forced browse](https://www.zaproxy.org/docs/desktop/addons/forced-browse/) attack to discover hidden files. While this attack is being executed, keep an eye out for interesting finds in the file system GUI. Oh, look!

![[Pasted image 20230807150021.png]]

This seems important. Open it up.

![[Pasted image 20230807150127.png]]
![[Pasted image 20230807150139.png]]

We also found a hidden registration page!

![[Pasted image 20230807150937.png]]

This seems different from the one you can find on the mutillidae website...

![[Pasted image 20230807151118.png]]

Take a look at more of these finds on your own before moving on to the next section.



### Metasploitable: Finding common files and directories using DirBuster
---

This process is similar to ZAP, where we are brute forcing a web site to discover hidden files and directories. To start, download DirBuster (e.g. `sudo apt install dirbuster`) and open it. Then, type in the following information (we'll be using our Metasploitable Mutillidae web app, you can use something else if you want, like [this](https://sourceforge.net/projects/owaspbwa/)):

![[Pasted image 20230809095330.png]]

Make sure that the information is correct, and click $\text{Start}$.

![[Pasted image 20230809095532.png]]

Go over the results! Keep in mind that since we have 200 threads running, the web server will experience some performance issues. Press $\text{Pause}$ if you are experiencing low speeds, or individually pause scans of your choosing!

![[Pasted image 20230810163622.png]]



### Metasploitable: Vulnerability scanning with Nikto 
---
[Nikto](https://www.geeksforgeeks.org/what-is-nikto-and-its-usages/) is a simple command line vulnerability scanner. Using it, you can discover common web application vulnerabilities and misconfigurations

![[Pasted image 20230809172510.png]]

Here's an interesting one! Let's test it out to see if it works.

![[Pasted image 20230809172229.png]]
![[Pasted image 20230809172551.png]]

Cool! However, we are on Level 0; it probably won't work for more secure web applications.

![[Pasted image 20230809172712.png]]
![[Pasted image 20230809172739.png]]




### Metasploitable: Vulnerability scanning with Nessus
---

Nessus is one of the most popular vulnerability scanners. Install it [here](https://www.tenable.com/downloads/nessus?loginAttempted=true).

![[Pasted image 20230809174923.png]]

By default, Nessus uses port 8834.

![[Pasted image 20230809175208.png]]
![[Pasted image 20230809175259.png]]

Keep the default settings and install the free version of Nessus.

![[Pasted image 20230809175456.png]]

Once everything has finished installing, you should be able to create a new scan. 

![[Pasted image 20230809182445.png]]

Set up a scan for your target web application.

![[Pasted image 20230809182937.png]]
![[Pasted image 20230809183425.png]]

Since I am using Mutillidae through metasploitable, I'll have to configure the assessment settings like so:

![[Pasted image 20230809183858.png]]
![[Pasted image 20230809183954.png]]

Save, then initiate the scan. While it's running, you'll be able to see the exposed vulnerabilities and take a closer look at the details by individually clicking on them.

![[Pasted image 20230809185429.png]]



### Metasploitable: Reflected XSS
---
$$\href{https://portswigger.net/web-security}{Web \ Security\ Cheat\ Sheets}$$
[Cross Site Scripting](https://owasp.org/www-community/attacks/xss/) (XSS) attacks are one of the most prevalent issues in web applications. Let's take a closer look at how these attacks can exploit user input vulnerabilities by covering reflected and stored XSS. In this section, we'll be covering reflected XSS, otherwise known as first-order XSS, or non-persistent.

First, navigate to the DVWA webpage and click on $\text{XSS Reflected}$ (make sure security is on $\text{low}$)

![[Pasted image 20230809114503.png]]

Try entering your name. 

![[Pasted image 20230809114532.png]]

Then enter:
$$\text{<script>alert("Example Reflected XSS attack")</script>}$$

![[Pasted image 20230809114826.png]]

As you can see, our input was "reflected" back to us.

Helpful:
- [<script\>](https://www.w3schools.com/tags/tag_script.asp)
- [alert()](https://www.w3schools.com/jsref/met_win_alert.asp)

Let's try a reflected XSS on Mutillidae with Security Level 0:

![[Pasted image 20230809124627.png]]
![[Pasted image 20230809124655.png]]
![[Pasted image 20230809124710.png]]

Now, toggle the security level and try again. 

![[Pasted image 20230809125002.png]]

Looks like the simple stuff won't work anymore. 

On Security Level 1, $\text{secret.php}$ shows that there is input validation in place.

![[Pasted image 20230810171818.png]]

Maybe Burpsuite will help?

![[Pasted image 20230810173738.png]]
![[Pasted image 20230810172128.png]]

Well that was easy...will this work for level 5?

![[Pasted image 20230810172230.png]]
![[Pasted image 20230810172252.png]]

It seems like Security Level 1 only has client side input security, meaning that the input is only checked on user input, and not when it is in transit or accepted to the web server. This problem is resolved in Level 5, though, as our input was checked on the server side and resulted in an error.

![[Pasted image 20230810175938.png]]
![[Pasted image 20230810175950.png]]

I wonder if this'll be useful...

After testing our script for a while, I was able to find that this:

![[Pasted image 20230810174342.png]]

resulted in an input error, but it modified the browser name at the bottom of the site as well.

![[Pasted image 20230810174443.png]]

Can we execute this somehow? Since I am all out of ideas, let's consult ChatGPT.

![[Pasted image 20230810175411.png]]
![[Pasted image 20230810175426.png]]
![[Pasted image 20230810175444.png]]
![[Pasted image 20230810175455.png]]

That took a bit of convincing, but we finally got the information we need. Since I don't know JavaScript, I'll have to take a closer look at these.

![[Pasted image 20230810180735.png]]
- [onload Event](https://www.w3schools.com/jsref/event_onload.asp)
Wow that seems very useful! Maybe it'll work. (It didn't)

![[Pasted image 20230810180719.png]]
- [onerror Event](https://www.w3schools.com/jsref/event_onerror.asp)
This seems promising.

![[Pasted image 20230810181051.png]]

Nope.

![[Pasted image 20230810181136.png]]
![[Pasted image 20230810181145.png]]

It worked! I almost missed it since there was a second proxied request for it. Try to complete reflected XSS on medium in DVWA! View the page source or source code if you are having trouble. After that, we'll continue with stored XSS.
$$\textit{P.S. We are using DVWA 1.0.7, where the 'High' security level}$$
$$\textit{actually goes by 'Impossible' in more recent versions. This should be}$$
$$\textit{protected against all vulnerabilities}$$
![[Pasted image 20230811195503.png]]



### Metasploitable: Stored XSS
---
$$\href{https://portswigger.net/web-security}{Web \ Security\ Cheat\ Sheets}$$
Now that you know the basics of reflected XSS, it's time we move on to stored XSS. This is also known as a second-order XSS, or persistent, for reasons you will see soon. You should be able to easily complete the first security level like so:


![[Pasted image 20230809140928.png]]

After signing with the same name again, we get:

![[Pasted image 20230809141045.png]]
![[Pasted image 20230809141137.png]]

As you can see, the script was stored on the server, and served to the user on request. Now, execute a stored XSS on medium security. Again, take a look at the source code or page source if you are stuck.

If you've made it this far, congrats! We learned that XSS attacks exploit the trust a browser has in a web server. Next, let's take a look at an attack that exploits the trust a web server has in a browser. 



### Metasploitable: CSRF
---
$$\href{[What is CSRF (Cross-site request forgery)? Tutorial & Examples | Web Security Academy (portswigger.net)](https://portswigger.net/web-security/csrf)}{Web\ Security\ Cheat\ Sheets}$$
[Cross Site Request Forgery](https://owasp.org/www-community/attacks/csrf) (CSRF) specifically targets authenticated users by executing malicious requests on a site where the user is logged in. To better understand this, let's use DVWA. 

![[Pasted image 20230811214618.png]]

DVWA provides a common scenario in which an attacker provides a link disguised as a normal website, but in the background, the user is clicking on a link that changes their password to a website. Intercept a request using Burpsuite.

![[Pasted image 20230811215526.png]]

This form uses a simple $\text{Get}$ request to change the password. Copy it so we can paste it later. Forward the request, and confirm that it was changed by logging out. 

![[Pasted image 20230811215811.png]]

Your new password should have worked! Now, use the link you copied instead of the password change form. Set a different password.

![[Pasted image 20230811215940.png]]

Logout and test the new password. Then, set the security level to medium and try again.

![[Pasted image 20230811220135.png]]

It didn't work...let's take a closer look at the differences in our requests.

![[Pasted image 20230811220915.png]]
![[Pasted image 20230811220846.png]]

It's missing a Referer header! 





















### WAFW00F: Web Application Firewall Detection
---

$$\textbf{\color{red}I have added bernard.local to my /etc/hosts file to map to the IP address of bernard}$$

As you know, we haven't implemented a Web Application Firewall ([WAF](https://www.cloudflare.com/learning/ddos/glossary/web-application-firewall-waf/)) for bernard yet. We can use `wafw00f` to confirm this.

![[Pasted image 20230807175939.png]]

While we can use OPNsense's [Nginx and Naxsi](https://medium.com/@jccwbb/website-protection-with-opnsense-3586a529d487) plugins to handle our WAF, let's first take a look at how we can use an Apache module ([ModSecurity](https://en.wikipedia.org/wiki/ModSecurity)) for bernard instead.
$$\text{sudo apt install libapache2-mod-security2}$$
![[Pasted image 20230808172218.png]]

We'll be installing [these](https://github.com/coreruleset/coreruleset/tree/v4.0/dev/rules) WAF rules from OWASP in the following directory:

![[Pasted image 20230808173946.png]]
![[Pasted image 20230808174348.png]]

Next, download the OWASP core rule set [here](https://coreruleset.org/installation/) and decompress it into our new directory.

$$\text{sudo tar xvzf [source\_path] -C [target\_path]}$$

![[Pasted image 20230808174911.png]]

Then, we'll need to change the names of the files with `.example` extension so that you can add updates without overwriting these files.

![[Pasted image 20230808175908.png]]
![[Pasted image 20230808175153.png]]

$$\textit{Note: it is important that you make changes to crs-setup.conf so that it better}$$
$$\textit{suits your purposes. For this project, we will leave it as default}$$

Finally, we need to add the following lines to our website `.conf` file. This ensures that the enclosed settings are only applied when the ModSecurity module loads correctly. We turn the security rule engine to on; alternatively, you can use `DetectionOnly` to generate logs. You can get more information about the syntax for the `crs-setup.conf` file [here](https://github.com/SpiderLabs/owasp-modsecurity-crs/blob/v3.3/dev/crs-setup.conf.example)

![[Pasted image 20230808174147.png]]
![[Pasted image 20230808182245.png]]

Now, restart your apache server and try some attacks from your attack VM. Resolve any errors.

![[Pasted image 20230808181148.png]]

It's working! Let's see what WAFW00F has to say.

![[Pasted image 20230808181257.png]]


