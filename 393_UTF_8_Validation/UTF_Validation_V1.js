

var validUtf8 = function (temp) {
    var convertUtf8 = function (data) {
        let biArray = data.map((num) => {
            let biNum = (num >>> 0).toString(2);
            if (biNum.length === 8) {
                return biNum;
            }
            else {
                return ("0".repeat(8 - biNum.length) + biNum);
            }
        })
        return biArray;
    }
    let remainder = 0;
    let dataArray = convertUtf8(temp);
    dataArray.forEach(function (data) {
        if (remainder === 0) {
            if (data.substr(0, 3) === '110') {
                remainder = 1;
            }
            else if (data.substr(0, 4) === '1110') {
                remainder = 2;
            }
            else if (data.substr(0, 5) === '11110') {
                remainder = 3;
            }
            else if (data.slice(-2) != 0) {
                return false;
            }
        }
        else {
            if (data.substr(0, 2) != '10') {
                return false;
            }
            else {
                remainder = remainder - 1;
            
            }
        }
    })
    return remainder === 0;
}