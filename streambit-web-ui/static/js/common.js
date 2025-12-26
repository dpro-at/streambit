// Common utility functions shared across all modules

// Format file size
function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}

// Format duration (seconds to MM:SS)
function formatDuration(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
}

// Show loading state
function showLoading(elementId, message) {
    const element = document.getElementById(elementId);
    if (element) {
        element.style.display = 'block';
        element.innerHTML = `
            <div style="text-align: center; padding: 30px;">
                <div style="font-size: 3em; margin-bottom: 15px;">
                    <i class="fas fa-spinner fa-spin" style="color: var(--accent-primary);"></i>
                </div>
                <div style="font-size: 1.2em; color: var(--text-primary);">${message}</div>
            </div>
        `;
    }
}

// Show error message
function showError(elementId, message) {
    const element = document.getElementById(elementId);
    if (element) {
        element.style.display = 'block';
        element.innerHTML = `
            <div style="background: rgba(255, 51, 102, 0.1); padding: 25px; border-radius: 12px; border: 2px solid var(--error); text-align: center;">
                <div style="font-size: 3em; color: var(--error); margin-bottom: 15px;">
                    <i class="fas fa-exclamation-triangle"></i>
                </div>
                <div style="font-size: 1.2em; color: var(--error);">${message}</div>
            </div>
        `;
    }
}

// Show success message
function showSuccess(elementId, message) {
    const element = document.getElementById(elementId);
    if (element) {
        element.style.display = 'block';
        element.innerHTML = `
            <div style="background: linear-gradient(135deg, rgba(0, 255, 136, 0.1) 0%, rgba(0, 212, 255, 0.1) 100%); padding: 25px; border-radius: 12px; border: 2px solid var(--success); text-align: center;">
                <div style="font-size: 3em; color: var(--success); margin-bottom: 15px;">
                    <i class="fas fa-check-circle"></i>
                </div>
                <div style="font-size: 1.2em; color: var(--success);">${message}</div>
            </div>
        `;
    }
}

// Switch between media tabs
function switchMedia(media) {
    document.querySelectorAll('.media-tab').forEach(tab => {
        tab.classList.remove('active');
    });
    document.querySelectorAll('.media-content').forEach(content => {
        content.classList.remove('active');
    });

    document.querySelector(`[data-media="${media}"]`).classList.add('active');
    document.getElementById(`${media}-content`).classList.add('active');
}

// Switch between sub-tabs
function switchTab(tab) {
    document.querySelectorAll('.sub-tab').forEach(t => {
        t.classList.remove('active');
    });
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.remove('active');
    });

    document.querySelector(`[data-tab="${tab}"]`).classList.add('active');
    document.getElementById(`${tab}-tab`).classList.add('active');
}
