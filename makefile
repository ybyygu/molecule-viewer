# [[file:ui.note::eb486478][eb486478]]
run: uic2py
	python main.py

uic2py:
	pyuic5 -o main_ui.py compt-platform.ui
# eb486478 ends here
