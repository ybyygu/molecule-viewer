# [[file:ui.note::e7246496][e7246496]]
from main_ui import *
from PyQt5.QtWidgets import QApplication, QMainWindow, QFileDialog
from PyQt5.QtWidgets import QTableWidgetItem
from PyQt5.QtWebEngineWidgets import QWebEngineView
from PyQt5 import QtCore


def update_webview_url(web: QWebEngineView, url: str):
    from PyQt5.QtCore import QUrl

    web.load(QUrl(url))


def get_wulff_html_path(plot_options):
    return "file:///home/path/to/wulff.html"


# 2 3 5:7 => 2,3,5-7
def jmol_selection_to_human_readable(s: str):
    return s.replace(":", "-").replace(" ", ",")


def gen_gosh_script(path, index, freeze_atoms: str = "", invert_selection=False):
    if len(freeze_atoms) > 0:
        s = jmol_selection_to_human_readable(freeze_atoms)
        freeze_atoms_commands = "\nselect {:}".format(s)
        if invert_selection:
            freeze_atoms_commands += "\nselect -u"
        else:
            freeze_atoms_commands += "\n"
    else:
        freeze_atoms_commands = "\n"

    template = """gosh -- << EOF
load {path}{freeze_atoms_commands}
write {index}.poscar
EOF"""

    return template.format(
        path=path,
        index=index,
        freeze_atoms_commands=freeze_atoms_commands,
    )


def update_web_widget_mol_file_path(
    web: QWebEngineView,
    mol_file_path: str,
    supercell=False,
    animation=False,
):
    from PyQt5.QtCore import QUrl
    from string import Template

    if animation:
        html = open("tests/jsmol/jsmol-anim-template.html").read()
    else:
        html = open("tests/jsmol/jsmol-template.html").read()

    html = html.replace("$mol_file_path", mol_file_path)
    if supercell:
        html = html.replace("$supercell", "supercell 3 3 1")
    else:
        html = html.replace("$supercell", "")

    path = "/home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/test1.html"
    with open(path, "w") as fp:
        fp.write(html)
    url = "file://{}".format(path)
    update_webview_url(web, url)


class MainForm(QMainWindow, Ui_MainWindow):
    def __init__(self):
        super(MainForm, self).__init__()
        self.setupUi(self)
        self.tableSurfaceFileList.itemDoubleClicked.connect(
            self.on_surface_table_item_clicked
        )
        self.tableHTCFileList.itemDoubleClicked.connect(self.on_htc_table_item_clicked)

    def show_status(self, msg):
        self.statusBar.showMessage(msg)

    @QtCore.pyqtSlot(name="on_btnUploadMolecule_clicked")
    def browse_jmol(self):
        file = self.editFilePath.text()
        # url = "file:///home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/supersimple2.htm"
        url = "file:///home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/supersimple2.htm"
        update_webview_url(self.webEngineView, url)
        self.statusBar.showMessage(url)

    @QtCore.pyqtSlot(name="on_btnSurfaceLoadCrystal_clicked")
    def surface_browse_jmol(self):
        # file = self.editFilePath.text()
        url = "file:///home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/supersimple2.htm"
        update_webview_url(self.webSurfaceShow, url)
        self.statusBar.showMessage(url)

    @QtCore.pyqtSlot(name="on_btnHTCBrowseFiles_clicked")
    def htc_browse_files(self):
        fnames = self.dialog_get_mol_files()
        s = "\n".join(fnames[0])
        self.editHTCFilePaths.setText(s)

    @QtCore.pyqtSlot(name="on_btnHTCLoadMolecules_clicked")
    def htc_load_molecules(self):
        paths = self.editHTCFilePaths.text().splitlines()
        if len(paths) > 0:
            table = self.tableHTCFileList
            n = len(paths)
            table.setRowCount(n)
            for (i, p) in enumerate(paths):
                table.setItem(i, 0, QTableWidgetItem(p))
            self.show_status("Loaded {} molecules.".format(n))
        else:
            self.show_status("No molecule to load.".format(len(paths)))

    @QtCore.pyqtSlot(name="on_btnHTCProjectRoot_clicked")
    def htc_set_project_root_directory(self):
        dir = self.dialog_get_project_directory()
        self.lineEditHTCProjectRoot.setText(dir)
        self.show_status("selected: {}".format(dir))

    @QtCore.pyqtSlot(name="on_btnHTCLoadProject_clicked")
    def htc_load_project_outputs(self):
        dir = self.lineEditHTCProjectRoot.text().strip()
        table = self.tableHTCOutputs
        table.setRowCount(1)
        table.setItem(0, 0, QTableWidgetItem(dir))

    @QtCore.pyqtSlot(name="on_btnHTCDOS_clicked")
    def htc_output_show_dos(self):
        web = self.webHTCOutputView
        url = "file:///home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/dos.html"
        update_webview_url(web, url)
        self.show_status("DOS created")

    @QtCore.pyqtSlot(name="on_btnHTCSaveScript_clicked")
    def htc_save_script(self):
        txt = self.plainTextHTCBatchScript.toPlainText()
        path, filter = QFileDialog.getSaveFileName(
            self, "Save file", "", "Text files (*.txt)"
        )
        if path:
            with open(path, "w") as f:
                f.write(txt)

    @QtCore.pyqtSlot(name="on_btnHTCGenBatchScript_clicked")
    def htc_gen_batch_script(self):
        text_widget = self.plainTextHTCBatchScript

        table = self.tableHTCFileList
        n = table.rowCount()
        lines = ["#! /usr/bin/env bash"]
        freeze_atoms = self.lineEditHTCFreezeAtoms.text()
        invert_selection = False
        if self.groupBoxHTCFreeze.isChecked() and freeze_atoms:
            freeze_atoms = freeze_atoms
            if self.checkBoxHTCInvertSelection.isChecked():
                invert_selection = True
        else:
            freeze_atoms = ""

        for i in range(n):
            path = table.item(i, 0).text()
            s = gen_gosh_script(
                path, i, freeze_atoms=freeze_atoms, invert_selection=invert_selection
            )
            lines.append(s)
            lines.append("bbm -t sp {}.poscar".format(i))
            lines.append("# input for job {} ends here\n".format(i))

        text_widget.setPlainText("\n".join(lines))
        self.show_status("batch script generated.")

    def on_htc_table_item_clicked(self, item: QTableWidgetItem):
        cif_file = item.text()
        update_web_widget_mol_file_path(self.webHTCMoleculeShow, cif_file)
        self.statusBar.showMessage("item clicked: {}".format(cif_file))

    @QtCore.pyqtSlot(name="on_btnSimulateXRD_clicked")
    def wulff_show_xrd(self):
        path = "/home/ybyygu/Workspace/Programming/structure-predication/ui/tests/jsmol/simulated_xrd.html"
        web = self.webXRD
        update_webview_url(web, "file://" + path)
        self.show_status("XRD graph created")

    @QtCore.pyqtSlot(name="on_btnBrowseFile_clicked")
    def wulff_open_mol_file(self):
        fname = self.dialog_get_mol_file()
        self.editFilePath.setText(fname[0])

    @QtCore.pyqtSlot(name="on_btnWulffProjectRoot_clicked")
    def wulff_set_project_root_directory(self):
        dir = self.dialog_get_project_directory()
        self.lineEditWulffProjectRoot.setText(dir)

    @QtCore.pyqtSlot(name="on_btnDrawWulff_clicked")
    def wulff_show_wulff_graph(self):
        self.wulff_show_table()
        # FIXME: read from UI widgets
        plot_options = ""
        url = get_wulff_html_path(plot_options)
        update_webview_url(self.webWulffShow, url)

    def wulff_show_table(self):
        table = self.tableSurfaceEnergies
        table.setRowCount(3)
        table.setItem(0, 0, QTableWidgetItem("(1,0,1)"))
        table.setItem(0, 1, QTableWidgetItem("3.22"))
        table.setItem(1, 0, QTableWidgetItem("(1,1,1)"))
        table.setItem(1, 1, QTableWidgetItem("2.24"))

    @QtCore.pyqtSlot(name="on_btnSurfaceBrowse_clicked")
    def surface_open_mol_file(self):
        fname = self.dialog_get_mol_file()
        self.editSurfaceFilePath.setText(fname[0])

    @QtCore.pyqtSlot(name="on_btnSurfaceCreateAdsorption_clicked")
    def surface_create_adsorption_structures(self):
        self.update_surface_table()
        self.show_status("clicked")

    # @QtCore.pyqtSlot(name="on_btnSetWullOptions_clicked")
    # def show_wulff_image(self):
    #     from PyQt5.QtGui import QPixmap

    #     pixmap = QPixmap("tests/wulff.png")
    #     self.lblWulff.setPixmap(pixmap)
    #     self.wulff_show_table()

    def update_surface_table(self):
        table = self.tableSurfaceFileList
        table.setRowCount(4)
        for i in range(4):
            table.setItem(i, 0, QTableWidgetItem("conf{}.cif".format(i)))

    def on_surface_table_item_clicked(self, item: QTableWidgetItem):
        cif_file = item.text()
        supercell = self.checkboxSurfaceSupercell.isChecked()
        update_web_widget_mol_file_path(
            self.webSurfaceShow, cif_file, supercell=supercell
        )
        self.statusBar.showMessage("item clicked: {}".format(cif_file))

    @QtCore.pyqtSlot(name="on_btnReactionPreview_clicked")
    def reaction_preview_path(self):
        web = self.webReactionPreview
        chain_file_path = "chain.cif"
        supercell = self.checkboxReactionSupercell.isChecked()
        update_web_widget_mol_file_path(
            web, chain_file_path, supercell=supercell, animation=True
        )
        self.statusBar.showMessage("item clicked")

    def dialog_get_mol_file(self):
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filters = "cif file (*.cif);;any file (*)"
        fname = QFileDialog.getOpenFileName(
            self, "Open file", ".", filters, options=options
        )
        return fname

    def dialog_get_mol_files(self):
        options = QFileDialog.Options()
        options |= QFileDialog.DontUseNativeDialog
        filters = "cif file (*.cif);;any file (*)"
        fname = QFileDialog.getOpenFileNames(
            self, "Open file", ".", filters, options=options
        )
        return fname

    def dialog_get_project_directory(self):
        return QFileDialog.getExistingDirectory(self, "Select Directory")

if __name__ == "__main__":
    import sys

    app = QApplication(sys.argv)
    win = MainForm()
    win.show()
    sys.exit(app.exec())
# e7246496 ends here
