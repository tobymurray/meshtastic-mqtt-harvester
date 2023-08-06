# meshtastic-mqtt-harvester

This program runs continuously to populate a database of publicly broadcast Meshtastic node positions. It does this by:

1. subscribing to the `msh/2/c/LongFast/#` topic on MQTT mqtt.meshtastic.org
2. decoding any messages that are coming from nodes publicly sharing their location
3. store the location (plus some other information) in the database

This is ultimately building the database to serve https://github.com/tobymurray/meshtastic-populator
