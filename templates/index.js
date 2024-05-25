document.getElementById('postButton').addEventListener('click', () => {
    const url = 'http://127.0.0.1:8000/long-laoshi/chat_with_ai';
    const data = {
        data: 'Enseñame algo'
    };

    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    })
    .then(response => response.json())
    .then(data => {
        console.log('Success:', data);
        document.getElementById('responseOutput').textContent = JSON.stringify(data, null, 2);
    })
    .catch((error) => {
        console.error('Error:', error);
        document.getElementById('responseOutput').textContent = `Error: ${error}`;
    });
});