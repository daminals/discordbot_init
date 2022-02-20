#!/bin/bash

if (source venv/bin/activate) ; then
    source venv/bin/activate
    echo "Venv Activated..."
else 
    python3 -m venv venv 
    source venv/bin/activate
    pip3 install -r requirements.txt
    cd ..
    echo "Venv Activated..."
fi

# add the python file.py here 
python3 bot.py