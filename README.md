# Zmeow
Very simple replacement for `zcat` built only to do one thing: read a compressed file and output its uncompressed contents to STDOUT

Given that it's aims are so simplistic, the main criteria for the design is to do so as fast as possible.

## Performance
On my computer (AMD Ryzen 9 3900X with 64GB RAM, high throughput NVME disks), I get about 420 MB/s throughput reading a 1.2MB gzipped file. `zcat` yields about 240 MB/s

Measurements performed using `cpipe` like so ```zcat ~/some_file.gz | cpipe -vt > /dev/null```

In the event you end up trying this and want to help out by measuring the performance on your machine, please file a GH issue with your system specs and measured performance vs `zcat`
