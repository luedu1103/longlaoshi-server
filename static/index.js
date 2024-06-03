document.getElementById('postButton').addEventListener('click', () => {
    const url = 'https://longlaoshi-server.shuttleapp.rs/longlaoshi/download-apk';

    fetch(url)
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.blob();
    })
    .then(blob => {
        const blobUrl = window.URL.createObjectURL(blob);
        
        // Create a temporary anchor element
        const a = document.createElement('a');
        a.href = blobUrl;
        a.download = 'app.apk'; // Set the filename
        a.style.display = 'none';
        
        document.body.appendChild(a);
        
        a.click();
        document.body.removeChild(a);
        window.URL.revokeObjectURL(blobUrl);
    })
    .catch(error => {
        console.error('Error downloading APK:', error);
        // Handle error
    });
});