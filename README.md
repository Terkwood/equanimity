# equanimity

bipolar mood charting utility

## build and run

```sh
MYIP=$(ip addr|grep 192|awk '{ print $2 }'|sed 's;\/[0-9]*;;')
sh build.sh && miniserve -i $MYIP ./dist --index index.html
```

## papers and articles

- http://www.bipolarnews.org/pdfs/Patient%20Prospective%20Manual.pdf
