var validUtf8 = function (dataArray) {
    let remainder = 0;
    dataArray.forEach(function (data) {
        if (remainder === 0) {
            if ((data >>> 5).toString(2) === '110') {
                remainder = 1;
            }
            else if ((data >>> 4).toString(2) === '1110') {
                remainder = 2;
            }
            else if ((data >>> 3).toString(2) === '11110') {
                remainder = 3;
            }
            else if ((data >>> 7).toString(2) !== 0) {
                return false;
            }
        }
        else {
            if ((data >>> 6).toString(2) !== '10') {
                return false;
            }
            else {
                remainder = remainder - 1;
            
            }
        }
    })
    return remainder === 0;
}
