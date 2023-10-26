export async function getSecure() {
    let res = await fetch('/secure',{credentials: 'same-origin'});
    let secureResponse = await res.json();
    return JSON.stringify(secureResponse.session);
} 

export async function getApi(api_token) {
    let res = await fetch('/api', {
        headers: {
            'Authorization': 'Bearer '+ api_token,
            Accept: "application/json", 
        },
    });
    return await res.json();
} 

export async function infosByTag(tagName, onSuccess){
    fetch("http://127.0.0.1:7070/youtube/tag/" + tagName, {
      method: 'GET',
    })
    .then(res => {
          console.log("res:" + res)
          return res.json()
    })
    .then(data => {
      onSuccess(data)
      // console.log("data:" + console.log(JSON.stringify(data)))
      //return data;//can json be a any type? not sure      
    })
    .catch(error => console.log('request error'+ error))
  }
