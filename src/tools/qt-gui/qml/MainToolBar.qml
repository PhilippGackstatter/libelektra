import QtQuick 2.2
import QtQuick.Controls 1.1
import QtQuick.Layouts 1.0

ToolBar {

	RowLayout {
		id:tbLayout

		anchors.fill: parent

		ToolButton {
			id:tbNew
			iconSource: "icons/document-new.png"
			enabled: newKeyAction.enabled
			menu: Menu {
				MenuItem {
					action: newKeyAction
				}
				MenuItem {
					action: newArrayAction
				}
			}
			HelpArea {
				helpText: qsTr("This is the button to create new keys/arrays. To create a key,\nprovide a name, a value and optional metadata. Array entries\nwill be named automatically.")
			}
		}
		ToolButton {
			id:tbDelete
			action: deleteAction
			HelpArea {
				helpText: qsTr("This button will delete the current key and associated values.\nIt will also delete the associated data from any configuration\nfiles mounted to this location upon synchronization.")
			}
		}
		ToolButton {
			id:tbImport
			action: importAction
			HelpArea {
				helpText: qsTr("This button allows you to import keys from a file. You can import\nmany types of files using Elektra plugins with this tool.")
			}
		}
		ToolButton {
			id:tbExport
			action: exportAction
			HelpArea {
				helpText: qsTr("This button allows you to export keys to a file. You can export\nmany types of files using Elektra plugins with this tool. The ecf\nfiletype is recommended if you want to preserve all aspects of\nthe current keys.")
			}
		}
		ToolButton {
			id:tbUndo
			action: undoAction
			HelpArea {
				helpText: qsTr("This button will undo the previous action.")
			}
		}
		ToolButton {
			id: tbUndoAll

			implicitWidth: defaultMargins
			enabled: undoAction.enabled
			menu: Menu {
				MenuItem {
					action: undoAllAction
				}
			}
		}
		ToolButton {
			id:tbRedo
			action: redoAction
			HelpArea {
				  helpText: qsTr("This button will redo the last action.")
			}
		}
		ToolButton {
			id: tbRedoAll

			implicitWidth: defaultMargins
			enabled: redoAction.enabled
			menu: Menu {
				MenuItem {
					action: redoAllAction
				}
			}
		}
		ToolButton {
			id:tbSynchronize
			action: synchronizeAction
			HelpArea {
				helpText: qsTr("This button will synchronize any changes you\nhave made with the Key Database. As such, if\nany changes were made to mounted config-\nuration files then those files will be updated\ntoo.")
			}
		}
		Item {
			Layout.fillWidth: true
			height: tbNew.height
		}
		Image {
			id: searchLogo
			source: "icons/edit-find.png"
		}
		SearchField {
			id: searchField

			implicitWidth: keyAreaWidth - searchLogo.implicitWidth - defaultMargins
			focus: true
			onAccepted: {
				if(text !== ""){
					searchResultsListView.model = treeView.treeModel.find(text)
					searchResultsListView.currentIndex = -1
					searchResultsListView.forceActiveFocus()
					searchResultsColorAnimation.running = true
				}
			}
			HelpArea {
				helpText: qsTr("This is the search field. Any searches here\nwill search the entire Key Database for\nmatching keys, values, or metadata.")
			}
		}
	}
}

