#!/bin/bash

sea-orm-cli migrate fresh
sea-orm-cli generate entity -u sqlite://test.sqlite -o entity/src -l
