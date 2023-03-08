# WER offline installer

Using the old Wizards Event Reporter is still possible offline, but on new computers after installation the application fails to start.
This is due to WotC shutting down the old backend, and the application attempts to communicate with it on first startup, even in offline mode.

This package contains:

- `wer.exe`, a local fake backend with barely enough functionality to mock the required endpoints and get WER started for the first time.
- `Reporter.exe.config`, modified WER config to connecto a local backend instead of the real one
- `countries`, XML files containing vital country / translation files, normally downloaded from WotC servers on first startup.
- `ReporterSetup.exe`, original WER installer, not included on the github repository.

The `wer.exe` tool is open source (MIT licensed), and the source code can be found from [Github](https://github.com/Cadiac/wer-offline-installer). The tool is intended for Windows,
but could probably be ported for other platforms too or hosted as a web backend for anyone to use.

This service is not affiliated with Wizards of the Coast and is used at your own risk.

## Installation steps

1. Run the original WER installer `ReporterSetup.exe` and follow install instructions. Don't launch WER yet.
2. Copy `Reporter.exe.config` to the install location, replacing the existing file.
3. Run the `wer.exe`. This setups a backend mocking the WotC backend just enough to get WER started, and copies `countries` XML files to their destination.
4. With wer.exe running launch Wizards Event Reporter ("Reporter.exe").
5. Choose your language
6. WER should give you a warning about "Update Server Unreachable" and "Server Unreachable" -> just click OK
7. If all goes well WER should now eventually launch. Verify that you can open the Options menu without seeing an error about countries.
8. You can stop the `wer.exe` by closing the terminal window, and you should no longer ever need it again.

After the initial setup WER should be able to launch offline without needing the wer.exe for startup again.

## Troubleshooting

### Error about SSL/TLS or services.onlinegaming.wizards.com:5901
Make sure wer.exe is running when you launch WER, and that you replaced the original Reporter.exe.config with the modified file within this installation package.

### WER starts, but after scheduling an event nothing happens and event isn't created
Check if you see an error about missing countries/en-us.xml file when you open the Options menu.
If yes, try running wer.exe again, it should create the missing countries files. If wer.exe fails to do this automatically
you can manually copy the files from "countries" folder to %AppData%\Wizards of the Coast\Event Reporter\countries

### Tool fails to copy countries files
Manually copy the XML files from "countries" folder to "C:\Users\<USERNAME>\AppData\Roaming\Wizards of the Coast\Event Reporter\countries"
