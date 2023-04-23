# make 1000 requests to specified URL 

url="http://slapkins.com/test.html"

make_request() {
    curl -s  "$url" > /dev/null    
    echo "Request completed"
}

export -f make_request

seq 10000 | parallel -j 12 make_request 

