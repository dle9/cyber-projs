# Author : Deric Le 
# Used to authenticate with Google OAuth
# and send an email.

import os.path
import base64

from email.message import EmailMessage
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.errors import HttpError

# If modifying these scopes, delete the file token.json.
SCOPES = ["https://www.googleapis.com/auth/gmail.readonly", "https://www.googleapis.com/auth/gmail.compose", "https://www.googleapis.com/auth/gmail.send"]


def authenticate():
  """Shows basic usage of the Gmail API.
  Lists the user's Gmail labels.
  """
  creds = None
  # The file token.json stores the user's access and refresh tokens, and is
  # created automatically when the authorization flow completes for the first
  # time.
  if os.path.exists("token.json"):
    creds = Credentials.from_authorized_user_file("token.json", SCOPES)
  # If there are no (valid) credentials available, let the user log in.
  if not creds or not creds.valid:
    if creds and creds.expired and creds.refresh_token:
      creds.refresh(Request())
    else:
      flow = InstalledAppFlow.from_client_secrets_file(
          "credentials.json", SCOPES
      )
      creds = flow.run_local_server(port=0)
    # Save the credentials for the next run
    with open("token.json", "w") as token:
      token.write(creds.to_json())

  return creds

def send_email(creds):
    _from = "example@gmail.com"
    _to = "example@gmail.com"

    try:
        # call Google API
        service = build("gmail", "v1", credentials=creds)
        print("Created service")

        message = EmailMessage()
        message.set_content("Hello from google API")
        message["To"] = _to
        message["From"] = _from
        message["Subject"] = "Google API lol"
        print("Initialized message")

        encoded_message = base64.urlsafe_b64encode(message.as_bytes()).decode()
        print("Encoded message")

        create_message = {"raw" : encoded_message}
        print("Created message")

        result = service.users().messages().send(userId="me", body=create_message).execute()
        print(f'Sent message with ID {result["id"]}')

    except HttpError as error:
        print(f'Error in sending email')

if __name__ == "__main__":
    print("Starting authentication...")
    creds = authenticate()
    print()
    print("==============")
    print("Authenticated!")
    print("==============")
    print()
    print("Sending email...")
    send_email(creds)
