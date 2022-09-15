var validUtf8 = function (dataArray) {
        let remainder = 0;
        for (let i = 0; i < dataArray.length; i++) {
            console.log(remainder);
            console.log(dataArray[i])
            console.log((dataArray[i] >>> 0).toString(2))
            if (remainder === 0) {
                if ((dataArray[i] >>> 5).toString(2) === '110') {
                    console.log((dataArray[i] >>> 5).toString(2) + " === 110: success, remainder = 1" )
                    remainder = 1;
                }
                else if ((dataArray[i] >>> 4).toString(2) === '1110') {
                    console.log((dataArray[i] >>> 4).toString(2) + "=== 1110: success, remainder = 2")
                    remainder = 2;
                }
                else if ((dataArray[i] >>> 3).toString(2) === '11110') {
                    console.log((dataArray[i] >>> 3).toString(2) + " === 11110: success, remainder = 3")
                    remainder = 3;
                }
                else if ((dataArray[i] >>> 7).toString(2) !== '0') {
                    console.log((dataArray[i] >>> 7).toString(2) + " != 0: success, exiting false")
                    return false;
                }
            }
            else {
                if ((dataArray[i] >>> 6).toString(2) != '10') {
                    console.log((dataArray[i] >>> 6).toString(2) + " != 10: success, exiting false")
                    return false;
                }
                else {
                    console.log("no case reached, decreminting remainder")
                    remainder = remainder - 1;
                
                }
            }
        }
        return remainder === 0;
    }