document.getElementById('postButton').addEventListener('click', () => {
    const url = 'https://longlaoshi-server.shuttleapp.rs/longlaoshi/download-apk';
    const data = {
        data: 'Enseñame algo'
    };

    fetch(url)
    .then(response => {
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        return response.blob();
    })
    .then(blob => {
        // Create a temporary URL for the blob
        const blobUrl = window.URL.createObjectURL(blob);
        
        // Create a temporary anchor element
        const a = document.createElement('a');
        a.href = blobUrl;
        a.download = 'app.apk'; // Set the filename
        a.style.display = 'none';
        
        // Append the anchor to the document body
        document.body.appendChild(a);
        
        // Programmatically click the anchor to trigger the download
        a.click();
        
        // Remove the anchor from the document body
        document.body.removeChild(a);
        
        // Revoke the temporary URL to free up memory
        window.URL.revokeObjectURL(blobUrl);
    })
    .catch(error => {
        console.error('Error downloading APK:', error);
        // Handle error
    });
});