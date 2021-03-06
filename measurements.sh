#!/bin/sh

export GST_PLUGIN_PATH="`pwd`/target/release"

for i in `seq 2 6`
do
    sar -P ALL 1 30 >measurements/proc_rs_${i}.txt &
    ./target/release/cctv-assistant rs $i | tee measurements/times_rs_${i}.txt && pkill -SIGINT sar
done

for i in `seq 2 6`
do
    sar -P ALL 1 30 >measurements/proc_opencv_${i}.txt &
    ./target/release/cctv-assistant opencv $i | tee measurements/times_opencv_${i}.txt && pkill -SIGINT sar
done