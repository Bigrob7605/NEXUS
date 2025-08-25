Write-Host "Completing git merge and pushing compression roadmap..." -ForegroundColor Green
Write-Host ""

Write-Host "Current git status:" -ForegroundColor Yellow
git status

Write-Host ""
Write-Host "Completing merge..." -ForegroundColor Yellow
git merge --continue

Write-Host ""
Write-Host "Adding all files..." -ForegroundColor Yellow
git add -A

Write-Host ""
Write-Host "Committing merge..." -ForegroundColor Yellow
git commit -m "Merge remote changes and add compression improvement roadmap"

Write-Host ""
Write-Host "Pushing to GitHub..." -ForegroundColor Yellow
git push origin main

Write-Host ""
Write-Host "Done! Check the output above for any errors." -ForegroundColor Green
Read-Host "Press Enter to continue"
