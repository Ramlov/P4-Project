#Scirpt made for testing the HTTP post request thingi
import requests
import json

url = 'https://app.ramlov.org/iot/test.php'

data = {
    "cmd": "rx",
    "seqno": 31,
    "EUI": "2CF7F12052602BFA",
    "ts": 1713521712759,
    "fcnt": 0,
    "port": 1,
    "freq": 867900000,
    "rssi": -108,
    "snr": 1.2,
    "toa": 1155,
    "dr": "SF12 BW125 4/5",
    "ack": False,
    "bat": 255,
    "offline": False,
    "data": "4545"
}

data1 = {
    "cmd": "gw",
    "seqno": 26,
    "EUI": "2CF7F12052602BFA",
    "ts": 1713521048021,
    "fcnt": 0,
    "port": 1,
    "freq": 867500000,
    "toa": 1155,
    "dr": "SF12 BW125 4/5",
    "ack": False,
    "gws": [
        {
            "rssi": -106,
            "snr": -4.5,
            "ts": 1713521048021,
            "time": "2024-04-19T10:04:08.021Z",
            "gweui": "7076FFFFFF053A13",
            "ant": 0,
            "lat": 57.013938,
            "lon": 9.9874619
        }
    ],
    "bat": 255,
    "data": "4545"
}


data2 ={"cmd":"txd","EUI":"2CF7F12052602BF1","seqdn":0,"seqq":1,"ts":1713521883960,"gweui":"7076FFFFFF053A13","freq":867700000,"sf":12,"codr":"4/5","toa":1155,"msgType":"User DL","time":1713521883960,"ackRequested":False,"port":1,"data":"01"}
payload = json.dumps(data2)

headers = {
    'Content-Type': 'application/json',
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36'
}

response = requests.post(url, data=payload, headers=headers)

print(response.text)
