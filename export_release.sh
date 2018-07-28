#!/bin/bash
mkdir export
cp target/release/dash-raid export/dash-raid
rsync -avp assets/* export/assets
zip -r dash-raid-export.zip export/*
