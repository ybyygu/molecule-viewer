# [[file:ui.note::e7246496][e7246496]]
from main_ui import *
from PyQt5.QtWidgets import QApplication, QMainWindow, QFileDialog
from PyQt5 import QtCore

class MainForm(QMainWindow, Ui_MainWindow):
    def __init__(self):
        super(MainForm, self).__init__()
        self.setupUi(self)

    def openMsg(self):
        pass

    def show_status(self, msg):
        self.statusBar.showMessage(msg)

    @QtCore.pyqtSlot(name="on_btnUploadMolecule_clicked")
    def browse_jmol(self):
        url = self.editFilePath.text()
        self.browse_url(
            "file:///home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/supersimple2.htm"
        )
        self.statusBar.showMessage(url)

    @QtCore.pyqtSlot(name="on_btnBrowseFile_clicked")
    def open_mol_file(self):
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filters = "cif file (*.cif);;any file (*)"
        fname = QFileDialog.getOpenFileName(self, 'Open file', '.', filters, options=options)
        self.editFilePath.setText(fname[0])

    def browse_url(self, url):
        from PyQt5.QtCore import QUrl

        self.webEngineView.load(QUrl(url))

    @QtCore.pyqtSlot(name="on_btnSetWullOptions_clicked")
    def show_wulff_image(self):
        from PyQt5.QtGui import QPixmap
        pixmap = QPixmap('tests/wulff.png')
        self.lblWulff.setPixmap(pixmap)

        self.show_wulff_table()

    def show_wulff_table(self):
        from PyQt5.QtWidgets import QTableWidgetItem
        table = self.tableSurfaceEnergies
        table.setRowCount(3)
        table.setItem(0, 0, QTableWidgetItem("(1,0,1)"))
        table.setItem(0, 1, QTableWidgetItem("3.22"))
        table.setItem(1, 0, QTableWidgetItem("(1,1,1)"))
        table.setItem(1, 1, QTableWidgetItem("2.24"))

if __name__ == "__main__":
    import sys

    app = QApplication(sys.argv)
    win = MainForm()
    win.show()
    sys.exit(app.exec())
# e7246496 ends here
