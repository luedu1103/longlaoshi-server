const postButton = document.getElementById('postButton');
// const loadingIndicator = document.getElementById('loadingIndicator');

postButton.addEventListener('click', async () => {
    const url = 'https://longlaoshi-server.shuttleapp.rs/longlaoshi/download-apk';
            const a = document.createElement('a');
            a.href = url;
            a.download = ''; // Let the server dictate the filename
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
});