// Audio Module - StreamBit Web UI

// Audio files storage
let audioFiles = [];
let audioFileInput = null;
let audioFileList = null;
let processAudioBtn = null;

// Initialize audio module
function initAudioModule() {
    audioFileInput = document.getElementById('audioFileInput');
    audioFileList = document.getElementById('audioFileList');
    processAudioBtn = document.getElementById('processAudioBtn');

    if (audioFileInput) {
        audioFileInput.addEventListener('change', handleAudioFiles);
    }

    displayAudioFiles();
}

// Handle audio file selection
function handleAudioFiles(e) {
    const files = Array.from(e.target.files);
    audioFiles = audioFiles.concat(files);
    displayAudioFiles();
}

// Display selected audio files
function displayAudioFiles() {
    if (!audioFileList) return;

    if (audioFiles.length === 0) {
        audioFileList.innerHTML = `
            <div style="text-align: center; padding: 30px; color: var(--text-secondary);">
                <i class="fas fa-music" style="font-size: 3em; margin-bottom: 15px; opacity: 0.3;"></i>
                <p>No audio files selected</p>
            </div>
        `;
        if (processAudioBtn) {
            processAudioBtn.disabled = true;
        }
        return;
    }

    let html = '<div style="display: flex; flex-direction: column; gap: 10px;">';

    audioFiles.forEach((file, index) => {
        const ext = file.name.split('.').pop().toUpperCase();
        const icon = getAudioIcon(ext);

        html += `
            <div style="display: flex; align-items: center; justify-content: space-between; padding: 12px; background: rgba(255, 255, 255, 0.05); border-radius: 8px; border: 1px solid var(--border-color);">
                <div style="display: flex; align-items: center; gap: 12px; flex: 1;">
                    <i class="${icon}" style="font-size: 1.5em; color: var(--accent-primary);"></i>
                    <div style="flex: 1; min-width: 0;">
                        <div style="font-weight: 500; color: var(--text-primary); overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${file.name}</div>
                        <div style="font-size: 0.85em; color: var(--text-secondary); margin-top: 2px;">
                            ${formatFileSize(file.size)} • ${ext}
                        </div>
                    </div>
                </div>
                <button onclick="removeAudioFile(${index})" style="background: rgba(255, 51, 102, 0.1); color: var(--error); border: none; padding: 8px 12px; border-radius: 6px; cursor: pointer; transition: all 0.3s;">
                    <i class="fas fa-times"></i>
                </button>
            </div>
        `;
    });

    html += '</div>';
    audioFileList.innerHTML = html;

    if (processAudioBtn) {
        processAudioBtn.disabled = false;
    }
}

// Get icon for audio file type
function getAudioIcon(ext) {
    const icons = {
        'MP3': 'fas fa-file-audio',
        'WAV': 'fas fa-waveform',
        'FLAC': 'fas fa-compact-disc',
        'OGG': 'fas fa-file-audio',
        'M4A': 'fas fa-file-audio',
        'AAC': 'fas fa-file-audio'
    };
    return icons[ext] || 'fas fa-file-audio';
}

// Remove audio file
function removeAudioFile(index) {
    audioFiles.splice(index, 1);
    displayAudioFiles();
}

// Process audio files
async function processAudioFiles() {
    if (audioFiles.length === 0) return;

    const results = document.getElementById('audio-results');

    if (processAudioBtn) {
        processAudioBtn.disabled = true;
        processAudioBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Processing...';
    }

    showLoading('audio-results', `Processing ${audioFiles.length} audio file(s)...`);

    const formData = new FormData();
    audioFiles.forEach(file => {
        formData.append('audio', file);
    });

    try {
        const response = await fetch('/process-audio', {
            method: 'POST',
            body: formData
        });

        const data = await response.json();

        if (data.success) {
            displayAudioResults(data);
        } else {
            showError('audio-results', data.message || 'Processing failed');
        }
    } catch (error) {
        showError('audio-results', 'Connection error: ' + error.message);
    } finally {
        if (processAudioBtn) {
            processAudioBtn.disabled = false;
            processAudioBtn.innerHTML = '<i class="fas fa-play"></i> Process Audio';
        }
    }
}

// Display audio processing results
function displayAudioResults(data) {
    const results = document.getElementById('audio-results');
    if (!results) return;

    results.style.display = 'block';

    let html = `
        <div style="background: linear-gradient(135deg, rgba(0, 255, 136, 0.1) 0%, rgba(0, 212, 255, 0.1) 100%); padding: 25px; border-radius: 12px; border: 2px solid var(--success); margin-bottom: 25px;">
            <div style="display: flex; align-items: center; gap: 15px; margin-bottom: 20px;">
                <div style="font-size: 3em; color: var(--success);">
                    <i class="fas fa-check-circle"></i>
                </div>
                <div>
                    <h3 style="margin: 0; color: var(--text-primary);">${data.message}</h3>
                    <p style="margin: 5px 0 0 0; color: var(--text-secondary);">Processed in ${data.time_ms.toFixed(2)}ms</p>
                </div>
            </div>
            
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value">${data.files_processed}</div>
                    <div class="stat-label">Files Processed</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${data.time_ms.toFixed(2)}ms</div>
                    <div class="stat-label">Processing Time</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${data.throughput.toFixed(2)}</div>
                    <div class="stat-label">Files/sec</div>
                </div>
            </div>
        </div>
    `;

    // Audio details
    if (data.audio_info && data.audio_info.length > 0) {
        html += `
            <div style="background: rgba(255, 255, 255, 0.03); padding: 20px; border-radius: 12px; border: 1px solid var(--border-color);">
                <h4 style="margin-top: 0; color: var(--text-primary);">
                    <i class="fas fa-info-circle"></i> Audio Information
                </h4>
                <div style="display: flex; flex-direction: column; gap: 15px;">
        `;

        data.audio_info.forEach((info, index) => {
            html += `
                <div style="padding: 15px; background: rgba(255, 255, 255, 0.05); border-radius: 8px; border: 1px solid var(--border-color);">
                    <div style="font-weight: 500; color: var(--text-primary); margin-bottom: 10px;">
                        <i class="fas fa-file-audio"></i> File ${index + 1}
                    </div>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; font-size: 0.9em;">
                        <div>
                            <span style="color: var(--text-secondary);">Sample Rate:</span>
                            <span style="color: var(--text-primary); font-weight: 500;"> ${info.sample_rate}Hz</span>
                        </div>
                        <div>
                            <span style="color: var(--text-secondary);">Channels:</span>
                            <span style="color: var(--text-primary); font-weight: 500;"> ${info.channels}</span>
                        </div>
                        <div>
                            <span style="color: var(--text-secondary);">Duration:</span>
                            <span style="color: var(--text-primary); font-weight: 500;"> ${info.duration.toFixed(2)}s</span>
                        </div>
                        <div>
                            <span style="color: var(--text-secondary);">Samples:</span>
                            <span style="color: var(--text-primary); font-weight: 500;"> ${info.samples.toLocaleString()}</span>
                        </div>
                    </div>
                </div>
            `;
        });

        html += `
                </div>
            </div>
        `;
    }

    results.innerHTML = html;
}

// Download audio dataset from Hugging Face
async function downloadAudioDataset() {
    const datasetSelect = document.getElementById('audio-dataset-select');
    const limitSelect = document.getElementById('audio-limit-select');
    const downloadBtn = document.getElementById('download-audio-btn');
    const statusDiv = document.getElementById('download-audio-status');

    if (!datasetSelect || !limitSelect) return;

    const dataset = datasetSelect.value;
    const limit = parseInt(limitSelect.value);

    if (downloadBtn) {
        downloadBtn.disabled = true;
        downloadBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Downloading...';
    }

    if (statusDiv) {
        statusDiv.style.display = 'block';
        statusDiv.innerHTML = `
            <div style="color: var(--accent-primary);">
                <i class="fas fa-spinner fa-spin"></i> Downloading ${limit} audio files from ${dataset}...
            </div>
        `;
    }

    try {
        const response = await fetch('/api/download-audio-dataset', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ dataset, limit })
        });

        const data = await response.json();

        if (data.success && statusDiv) {
            statusDiv.innerHTML = `
                <div style="color: var(--success);">
                    <i class="fas fa-check-circle"></i> ${data.message}
                </div>
            `;

            // Auto-switch to audio upload tab after 2 seconds
            setTimeout(() => {
                switchTab('audio-upload');
            }, 2000);
        } else if (statusDiv) {
            statusDiv.innerHTML = `
                <div style="color: var(--error);">
                    <i class="fas fa-exclamation-triangle"></i> ${data.message || 'Download failed'}
                </div>
            `;
        }
    } catch (error) {
        if (statusDiv) {
            statusDiv.innerHTML = `
                <div style="color: var(--error);">
                    <i class="fas fa-exclamation-triangle"></i> Error: ${error.message}
                </div>
            `;
        }
    } finally {
        if (downloadBtn) {
            downloadBtn.disabled = false;
            downloadBtn.innerHTML = '<i class="fas fa-download"></i> Download Dataset';
        }
    }
}

// Initialize when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initAudioModule);
} else {
    initAudioModule();
}
