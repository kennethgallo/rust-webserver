# make 10000 requests to specified URL 

make_request() {
    url="http://slapkins.com/test.html"
    curl -s  "$url" > /dev/null    
}

export -f make_request

seq 10000 | parallel -j 24 make_request 

    