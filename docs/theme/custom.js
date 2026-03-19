// Custom JavaScript for AgentVerse Documentation
// This file is included on every page

document.addEventListener('DOMContentLoaded', function() {
    // Make the book title link back to the main site
    var title = document.querySelector('h1.menu-title');
    if (title) {
        title.style.cursor = 'pointer';
        title.addEventListener('click', function() {
            window.location.href = 'https://agentvrs.com';
        });
    }
});
