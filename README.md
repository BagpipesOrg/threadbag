# Threadbag  

Threadbag - Persistence execution for Bagpipes

```ascii
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMN0OkO0NMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMKxc';oXMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWKOl':0WMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNd,'';dXMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM0d;,xodNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM0d:'odl0MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMk:;:l;lKMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMKl;;od,cXMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWx;;;od,oWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNo,::dd,dWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMXo:clddckWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWKkO0OokxkWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMNkllxOXWkkWMMMMMMMMMMMMNNNWWMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMXd,,cxNWxdWMMMMMMMMMMMXdldxKMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWO::oddxldWMMMMMMMMMMMK:':dXMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMXXWMMMMMMMMMMMMMMMMMMMMMM0c:oxKXx;lXMMWMMMMMMKxc,:ONMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMM0o0MMMMMMMMMMMMMMMMMMMMMWx;;okXMWd;dOKXXXKOkkd:,;ldKWMMMMMMMMMMMMWNWWWMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMWkoKMMMMMMMMMMMMMMMMMMMMNd,:xXNMNllK0OOkkOOOKk:c0NdoXMMMMMMMMMMMNxoxx0WMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMNddNMMMMMMMMMMMMMMMMMMMXo:lOWWWk',OWMMMMMMNOccOWMXdo0NMMMMMMWWNk;':oKWMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMKoxNMMMMMMMMMMMMMMMMMWKkkOXMMWx..xWMMMMMXo:;oXMMMN0xxOOOOOkkx:,;xKNMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMOckWMMMMMMMMMMMMMMMMW0llONMMWo..oNMMMMNd;,cOWMMMMMMNX0O0KKkl:ccxWMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMNo:kWMMMMMMMMMMMMMMMM0:cKMWMNc..:KMMMWO:,:kWMMMMMMMMMMMN0o:lONxlKMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMKlcKMMMMMMMMMMMMMMMMO:dNMWMX:..,0WMWXOdoOWMMMMMMMMMMWOo:;oXWMOl0MMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMM0coXMMMMMMMMMMMMMMWk:kWMWWK;..'kMMXdokKWMMMMMMMMMMXd:,;oKMMMOl0MMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMWkcxNMMMMMMMMMMMMMNoc0MMWMO,...xWWk;lKWMMMMMMMMMW0l;,cONMMMMOoKMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMNdckNMMMMMMMMMMMW0loKMMMMXdcco0WO:cKMMMMMMMMMMNOxdoxXWMMMMMOdXMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMKlc0WMMMMMMMMMMNkdkXWMMMMMWWWW0cc0WMMMMMMMMMXd:oOXWMMMMMMMOxNMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMWOclKWMMMMMMMMMXo:xXMMMMMMMMN0dlOWMMMMMMMMW0c;dKWMMMMMMMMMxdNMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMNxlkXWMMMMMMMMO:;xNMMMMMMMW0ddONMMMMMMMMXd:l0WMMMMMMMMMMWxdWMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMW0odOXMMMMMMMWx,:kWMMMMMMWOc:xXMMMMMMWXOo:xNMMMMMMMMMMMMNodWMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMWOclONMMMMMMXo:lOWMMMMMW0:,c0MMMMMMW0dod0WMMMMMMMMMMMMMNodWMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMMMMMMWWWNKkddkKXXNNN0lco0WMMMMW0:':kWMMWWN0o;:xKWMMMMMMMMMMMMMW0clXMMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMMMWNX0OkkxddooooooddxxdlclOXWWMW0l;;d00OOOxl,';lkKXWWMMMMMMMMMMMWd.,kWMMMMMMMMMM
MMMMMMMMMMMMMMMMMMMMWKkolccclollollollllooooooodxkOxc;:oddoool;',:looodk0XWMMMMMMMMMWd..xWMMMMMMMMMM
MMMMMMMMMMMMMMMMMMWKxlccccc::cc::c::c::::lllllloooolc;;clll:,;ccclloodddxkKWMMMMMMMMXc..cXMMMMMMMMMM
MMMMMMMMMMMMMMMMMNOc:::cc::;,;:,;:,';;,,;;ccllloooddoolcclc'..;ccccloodxxdkXMMMMMMMMK;..;0MMMMMMMMMM
MMMMMMMMMMMMMMMMWkc;;::::lodddl,,,..''.'',;;:::clllooooolloc;;;;:::coodddodKWMMMMMMM0,..'OMMMMMMMMMM
MMMMMMMMMMMMMMMMKc,,,;cd0NWWMWNOc........',,,;;::::cllllllloc::;:cc:clloold0WMMMMMMMO'...xWMMMMMMMMM
MMMMMMMMMMMMMMMNx:,;;cONMMMMMMMMXl.........''',;,;;::ccccccllc::;;::::cclldKWMMMMMMMx....dWMMMMMMMMM
MMMMMMMMMMMMMMMKxollo0WMMMMMMMMMMKc'..........',',,;;;;;::::c:;;;,;;;;:cclkNMMMMMMMM0c;;:OWMMMMMMMMM
MMMMMMMMMMMMMMW0lcodkXMMMMMMMMMMMMKc.............',,,,,,;,,;;;;,,,,;;;:ccxNMMMMMMMMMMWNNWWMMMMMMMMMM
MMMMMMMMMMMMMMMOlookNWMMMMMMMMMMMMMKo,...........'''',',,,,;;;;;;;;:::clkNMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMWx:lxXMMMMMMMMMMMMMMMMW0l,.......'''',,,,;;;;:::::::::cox0WMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMMk,cKWMMMMMMMMMMMMMMMMMMWXkoc;'',,,,;,,;;;;;::::ccldxOKNWMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMWxc0MMMMMMMMMMMMMMMMMMMMMMMMWX0OkxxdddddddxkkO00KXNWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMXldNMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMMOcOWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMNolXMMMMMMMMMMMMMMMThreadBagMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
MMMMMMMMMMMMMOckWMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM


```


Multi-threaded Persistence work server for Bagpipes. 


Made with: A Tokio runtime, Polodb + Sled, Subxt and actix-web. 


Backwards compatible with Bagpipes API   


## Run tests:
```shell
$ cargo test -- --nocapture
```

## How-to:  
-  Make a scenario with Bagpipes main UI     
-  Copy the scenario id from the scenario link   
-  Start the scenario job worker with your scenario id  
-  Query for logs and transactions in the que and sign and submit them on your side.   


### Test with python scripts:  
```shell  
root@computer /tmp/threadbag # python3.7 test_req.py 
Creating scenario..
Response: 
{"success":true,"shortUrl":"zvZLEgRyk"}
Scenario id saved as: zvZLEgRyk
Starting job: 
start job response: {'success': True, 'result': 'Job started'}
sleeping for 10 seconds.. 
Quering logs for:  zvZLEgRyk
Logs returned:  {"success":true,"result":["Starting worker","Decoding payload..","Parsed scenario data","Making http request: Url: https://example.com Method: post","Building http request","Building ChainNode request","Drafting xTransfer tx from polkadot to assetHub","0xe804630903000100a10f0300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e450304000000000c0000000000","Building Action request","Building ChainNode request","Building ChainNode request","Drafting xTransfer tx from assetHub to polkadot","0xdc041f090301000300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e45030400010000080000000000","Building Action request","workload executed","Sleeping"]}
Getting transaction queue:  {"mempool":[{"chain":"polkadot","amount":"3","txType":"xTransfer","Date":"2024-04-29T16:28:26.808100957Z","tx":"0xe804630903000100a10f0300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e450304000000000c0000000000"},{"chain":"assetHub","amount":"2","txType":"xTransfer","Date":"2024-04-29T16:28:27.536623019Z","tx":"0xdc041f090301000300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e45030400010000080000000000"}]}

```


## Uri's:   

#### xcm_asset_transfer     
TODO 



#### get_url   
This is the same function as [Bagpipes api](https://github.com/XcmSend/api) has.   

###### Path: ``    

###### Code example:  

###### Response:   




#### scenario_info     
###### Path: ``    

###### Code example:  

###### Response:   



#### save_url     
This is the same function as [Bagpipes api](https://github.com/XcmSend/api) has. This save the encoded scenario into the database.    
###### Path: `/saveUrl`    

###### Code example:  
```python  
# http > chain > xtransfer > chain
http_node = "eJzdV9tS4zgQ/ZUpPyfGF8mXvIVwmV0GpobAMsvWFCXbiuOJbWVkGQhU/n1bvscEqOzCVi1PkVut7laf063Oo5KygGbK6K9HJQqUkTIXYnljejamM2WgiNWSVkL4WrIsEhFLldGjcq+MhpZrqJZraq7mYNOybWugrECsa6auai42TNvCru4Y9nqgBEQQeS4mHo1rkyDPxCoGF4+wvIsCMVdGjjZQ5jQK56JcZzSmvqAQnOA5baMYexmLc0H/UTSchGGUhspoRuIMjM4YTw6qEDPKIxJHD/SSy0hTJq9OeEbPabZkaSYzsoKcQWhzwumEsUUkU1hqcvoTwr1Mb8HKLKLBhHIBC5+IQqc8OGNxzO7OaRBxUO7Lx3Hc3arM/sppJiYsWXKaZWCXpYKmojmaZ/Q0FzmJL75MGyG9JXEOng85Z7yxlfMagWy0t0fvSbKMqeqzBPYSKuZMEgHSLODbZ2kKcUC+L1ounJfBKOv1oKKNPydRehP41CNuy5tCuoU4yDBVV9OwbWJk6RrWS6hc5KiOgy2MXFdDDna28GZSmYwSEkoX6l7hRM1uQ5CmJKGN0ia7ugiXcck7xgsSMHlPAjkVcq8yUW8dtLu/wUUlH1eJx2QoB18vpA8SBBIREOCjL/h8fCnu59//DNjp8Xl6hhEPr06uDWIm7P5wuvh5TdIj/jn0ro6m36ThhOUFiCZ8BDQmkIc0j+Mi8YL4ovxsq8NAuC0PhMxufVRcfq5Adsn6Zn00MJOCCTdeQGdBpz+U4i1A27Zq2hg7pu3oOlQibmrSVhEyNKQbFtSlYW5BujL6PIqVwki5v+AkzWaUy4QWwk2VirldtYzl3KcvcaFC++3Q7fNGEB6WlKtDKPx+zr1uCIquKf8yisbx/uF4Cq7XHTZpnWZrYG0HNu0E7TN0KruGYRMXWa92DWjgqgYmsWM7JjJ016j4a5oqchCGYgCxhd67afRR6jSNU65+2qckE58mrHDYg7GPxFvRy3j35rFL8l8EW3dt5ONXwXZ1pNrAIAMebVt33IZd0Kxc13BsZGAsf/9vT8Tx/u1DtDybk8X+7/gsiELx7ezs1Dm5GP86Ol46q3FwjDFH19Pvqz/ECeugXCL7vjDvlPaX34gg0JGPXn8jsItV3XSxo2OkQTsxa5e2q9ouhskNW9Bktg6R//ET8U79eaOMn7aIp+/E60/V7izbpOybPBE7Idsj0w+YXYPwmT8mw960WSPV//NSJa4/nNZcgQVnC3ku5JSmxY4UXFUXh7ASwheUH6aBVK+JDLP0XV9ZXzdEzyI5Tnv0ISpoVHO1Xk7n7G4/rP/MlKI6II/4i5ADG2C6jxlvIitY7ecZ8aSePLl18B72x7MmMf3xvM7MkwMfKTUblxv2Zo0mNU9G2k3WtAc+Umq6dxv2G3aPNe141mNNe+AjpWbjcsPe0NJnTfvIbbKmPfARUvNj/Teu7M64"

## Upload a scenario to threadbag
print("Creating scenario..")
xx = requests.post(base+"/saveUrl", json={"url": http_node})
print("Response: ")
print(xx.text)
s = xx.json()
# save the scenario id
scenario_id = s.get('shortUrl', 'not found')
print("Scenario id saved as:", scenario_id)
```

###### Response:   
```json
{"success":true,"shortUrl":"zvZLEgRyk"}
```



#### get_logs    
Return the logs from a scenario worker   

###### Path: `/scenario/worker/logs`     

###### Code example:  
```python
logs = requests.post(base+"/scenario/worker/logs", json={"id": scenario_id})
print("Logs returned: ", logs.text)
```

curl:    
`curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWvg"}' http://localhost:8081/scenario/worker/logs -v`

###### Response:   
```json  
{"success":true,"result":["Starting worker","Decoding payload..","Parsed scenario data","Making http request: Url: https://example.com Method: post","Building http request","Building ChainNode request","Drafting xTransfer tx from polkadot to assetHub","0xe804630903000100a10f0300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e450304000000000c0000000000","Building Action request","Building ChainNode request","Building ChainNode request","Drafting xTransfer tx from assetHub to polkadot","0xdc041f090301000300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e45030400010000080000000000","Building Action request","workload executed","Sleeping"]}

```


#### broadcast_tx     
TODO  





#### dot_openchannels    
TODO  




#### scenario_transactions  // mempool      
All transactions a scenario generates is avaliable in this transaction queue.   

###### Path: `/scenario/tx`    

###### Code example:  
```python  
## Query for the transaction que 
txmempool = requests.post(base+"/scenario/tx", json={"id": scenario_id})

print("Getting transaction queue: ", txmempool.text)
```

###### Response:   
```json
{"mempool":[{"chain":"polkadot","amount":"3","txType":"xTransfer","Date":"2024-04-29T16:28:26.808100957Z","tx":"0xe804630903000100a10f0300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e450304000000000c0000000000"},{"chain":"assetHub","amount":"2","txType":"xTransfer","Date":"2024-04-29T16:28:27.536623019Z","tx":"0xdc041f090301000300010100c63c1fb2c2d4a97b9aa07b951159b273e0d6a740914f71c074a93499d10e3e45030400010000080000000000"}]}
```




#### start_job     
Start a scenario worker, after you have started the scenario, you will be able to query for logs and transactions in que.   
###### Path: `/job/start`    

###### Code example:  
```python 
print("Starting job: ")
js = requests.post(base+"/job/start", json={"id": scenario_id})
print("start job response:", js.json())
```

###### Response:   


#### info     
###### Path: ``    

###### Code example:  

###### Response:   


#### list_single_thread     
###### Path: ``    

###### Code example:  

###### Response:   

