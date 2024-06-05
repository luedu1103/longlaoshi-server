const postButton = document.getElementById('postButton');
const loadingIndicator = document.getElementById('loadingIndicator');

postButton.addEventListener('click', async () => {
    loadingIndicator.style.display = 'block'; // Show loading indicator

    const url = 'https://longlaoshi-server.shuttleapp.rs/longlaoshi/download-apk';

    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const blob = await response.blob();

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

        loadingIndicator.style.display = 'none'; // Hide loading indicator
    } catch (error) {
        console.error('Error downloading APK:', error);
        // Handle error
        loadingIndicator.style.display = 'none'; // Hide loading indicator on error
    }
});