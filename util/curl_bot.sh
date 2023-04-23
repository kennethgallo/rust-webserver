# make 1000 requests to specified URL 

url="http://slapkins.com/test.html"

for i in {1..1000}
do
    curl -s  "$url" > /dev/null  
done
