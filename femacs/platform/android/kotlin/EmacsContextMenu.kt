// femacs: port of java/org/gnu/emacs/EmacsContextMenu.java @ fd7afced403.

package org.femacs

import java.util.concurrent.Callable
import java.util.concurrent.FutureTask

import android.os.Build

import android.view.ContextMenu
import android.view.Menu
import android.view.MenuItem
import android.view.SubMenu

/* Context menu implementation.  This object is built from JNI and
   describes a menu hierarchy.  Then, `inflate' can turn it into an
   Android menu, which can be turned into a popup (or other kind of)
   menu.  */

/* `androidmenu.c` 76-118 finds this class and calls seven of its members by name and descriptor:
   the static `createContextMenu` `(Ljava/lang/String;)Lorg/gnu/emacs/EmacsContextMenu;`, and
   `addItem` `(ILjava/lang/String;ZZZLjava/lang/String;Z)V`, `addSubmenu`
   `(Ljava/lang/String;Ljava/lang/String;)Lorg/gnu/emacs/EmacsContextMenu;`, `addPane`
   `(Ljava/lang/String;)V`, `parent` `()Lorg/gnu/emacs/EmacsContextMenu;`, `display`
   `(Lorg/gnu/emacs/EmacsWindow;III)Z` and `dismiss` `(Lorg/gnu/emacs/EmacsWindow;)V` - so those
   seven keep the reference's names and parameter lists, and the static factory carries
   `@JvmStatic` (P-83 ⑪).  `parent ()` stays a *function* beside the `parent` property (Kotlin
   allows that pair; measured).  The three static fields `EmacsActivity` 508-529 reads as fields
   are companion `@JvmField`s (P-83 ②, the `EmacsApplication.dumpFileName` shape).  The
   reference's anonymous `Runnable`/`Callable`/`MenuItem.OnMenuItemClickListener` are Kotlin SAM
   literals (P-83 ①), and Java's unused `import`s (`android.content.Context`,
   `android.view.View`, `android.util.Log`) are not carried, nor is the `Intent intent;` local
   `inflateMenuItems` declares and never reads (Kotlin warns on an unused local; the constant
   `TAG`, which is unused in the reference too, is kept as Kotlin allows an unused private
   constant silently).  The reference's `java.util.List`/`ArrayList` imports are not carried:
   Kotlin's own `List`/`ArrayList` are the same JVM types, while importing the Java ones explicitly
   makes `ArrayList<Item>` no longer assignable to `List<Item>` (measured).  `Item` is `private`
   in the reference; Kotlin refuses a public field
   (`menuItems`, which `EmacsView` 719 reads) whose type argument is private-in-class, so the
   port's nested class is public - its JVM name `EmacsContextMenu$Item` and its members are
   unchanged, and no JNI lookup names it.  */
class EmacsContextMenu {
  companion object {
    private const val TAG = "EmacsContextMenu"

    /* Whether or not an item was selected.  */
    @JvmField var itemAlreadySelected: Boolean = false

    /* Whether or not a submenu was selected.
       Value is -1 if no; value is -2 if yes, and a context menu
       close event will definitely be sent.  Any other value is
       the timestamp when the submenu was selected.  */
    @JvmField var wasSubmenuSelected: Long = 0

    /* The serial ID of the last context menu to be displayed.  */
    @JvmField var lastMenuEventSerial: Int = 0

    /* Create a context menu with no items inside and the title TITLE,
       which may be NULL.  */

    @JvmStatic
    fun createContextMenu(title: String?): EmacsContextMenu {
      val menu: EmacsContextMenu

      menu = EmacsContextMenu()
      menu.title = title
      menu.menuItems = ArrayList<Item>()

      return menu
    }
  }

  /* The last group ID used for a menu item.  */
  @JvmField var lastGroupId: Int = 0

  class Item : MenuItem.OnMenuItemClickListener {
    @JvmField var itemID: Int = 0
    @JvmField var itemName: String? = null
    @JvmField var tooltip: String? = null
    @JvmField var subMenu: EmacsContextMenu? = null
    @JvmField var isEnabled: Boolean = false
    @JvmField var isCheckable: Boolean = false
    @JvmField var isChecked: Boolean = false
    @JvmField var inflatedView: EmacsView? = null
    @JvmField var isRadio: Boolean = false

    override fun onMenuItemClick(item: MenuItem): Boolean {
      if (subMenu != null) {
        /* Android 6.0 and earlier don't support nested submenus
           properly, so display the submenu popup by hand.  */

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N) {
          /* Still set wasSubmenuSelected -- if not set, the
             dismissal of this context menu will result in a
             context menu event being sent.  */
          wasSubmenuSelected = -2

          /* Running a popup menu from inside a click handler
             doesn't work, so make sure it is displayed
             outside.  */

          inflatedView!!.post(Runnable {
            inflatedView!!.popupMenu(subMenu!!, 0, 0, true)
          })

          return true
        }

        /* After opening a submenu within a submenu, Android will
           send onContextMenuClosed for a ContextMenuBuilder.  This
           will normally confuse Emacs into thinking that the
           context menu has been dismissed.  Wrong!

           Setting this flag prompts EmacsActivity to only handle
           SubMenuBuilders being closed, which always means the menu
           has actually been dismissed.

           However, these extraneous events aren't sent on devices
           where submenus display without dismissing their parents.
           Thus, only ignore the close event if it happens within
           300 milliseconds of the submenu being selected.  */
        wasSubmenuSelected = System.currentTimeMillis()
        return false
      }

      /* Send a context menu event.  */
      EmacsNative.sendContextMenu(0, itemID, lastMenuEventSerial)

      /* Say that an item has already been selected.  */
      itemAlreadySelected = true
      return true
    }
  }

  /* List of menu items contained in this menu.  Java's `List<Item>` is a list this class adds to
     (`addItem`/`addPane`/`addSubmenu`), so the port declares it `MutableList` - the descriptor is
     `Ljava/util/List;` either way, and the read-only Kotlin `List` would have no `add`
     (measured).  */
  @JvmField var menuItems: MutableList<Item>? = null

  /* The parent context menu, or NULL if none.  */
  private var parent: EmacsContextMenu? = null

  /* The title of this context menu, or NULL if none.  */
  private var title: String? = null

  /* Add a normal menu item to the context menu with the id ITEMID and
     the name ITEMNAME.  Enable it if ISENABLED, else keep it
     disabled.

     If this is not a submenu and ISCHECKABLE is set, make the item
     checkable.  Likewise, if ISCHECKED is set, make the item
     checked.

     If TOOLTIP is non-NULL, set the menu item tooltip to TOOLTIP.

     If ISRADIO, then display the check mark as a radio button.  */

  fun addItem(itemID: Int, itemName: String?, isEnabled: Boolean,
              isCheckable: Boolean, isChecked: Boolean,
              tooltip: String?, isRadio: Boolean) {
    val item: Item

    item = Item()
    item.itemID = itemID
    item.itemName = itemName
    item.isEnabled = isEnabled
    item.isCheckable = isCheckable
    item.isChecked = isChecked
    item.tooltip = tooltip
    item.isRadio = isRadio

    menuItems!!.add(item)
  }

  /* Create a disabled menu item with the name ITEMNAME.  */

  fun addPane(itemName: String?) {
    val item: Item

    item = Item()
    item.itemName = itemName

    menuItems!!.add(item)
  }

  /* Add a submenu to the context menu with the specified title and
     item name.  */

  fun addSubmenu(itemName: String?, tooltip: String?): EmacsContextMenu {
    val submenu: EmacsContextMenu
    val item: Item

    item = Item()
    item.itemID = 0
    item.itemName = itemName
    item.tooltip = tooltip
    item.subMenu = createContextMenu(itemName)
    item.subMenu!!.parent = this

    menuItems!!.add(item)
    return item.subMenu!!
  }

  /* Add the contents of this menu to MENU.  Assume MENU will be
     displayed in INFLATEDVIEW.  */

  private fun inflateMenuItems(menu: Menu, inflatedView: EmacsView) {
    var menuItem: MenuItem
    var submenu: SubMenu

    for (item in menuItems!!) {
      if (item.subMenu != null) {
        /* This is a submenu.  On versions of Android which
           support doing so, create the submenu and add the
           contents of the menu to it.

           Note that Android 4.0 and later technically supports
           having multiple layers of nested submenus, but if they
           are used, onContextMenuClosed becomes unreliable.  */

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
          submenu = menu.addSubMenu(item.itemName!!)
          item.subMenu!!.inflateMenuItems(submenu, inflatedView)

          /* This is still needed to set wasSubmenuSelected.  */
          menuItem = submenu.getItem()
        } else
          menuItem = menu.add(item.itemName!!)

        item.inflatedView = inflatedView
        menuItem.setOnMenuItemClickListener(item)
      } else {
        if (item.isRadio)
          menuItem = menu.add(++lastGroupId, Menu.NONE, Menu.NONE,
                              item.itemName!!)
        else
          menuItem = menu.add(item.itemName!!)
        menuItem.setOnMenuItemClickListener(item)

        /* If the item ID is zero, then disable the item.  */
        if (item.itemID == 0 || !item.isEnabled)
          menuItem.setEnabled(false)

        /* Now make the menu item display a checkmark as
           appropriate.  */

        if (item.isCheckable)
          menuItem.setCheckable(true)

        if (item.isChecked)
          menuItem.setChecked(true)

        /* Define an exclusively checkable group if the item is a
           radio button.  */

        if (item.isRadio)
          menu.setGroupCheckable(lastGroupId, true, true)

        /* If the tooltip text is set and the system is new enough
           to support menu item tooltips, set it on the item.  */

        if (item.tooltip != null
            && Build.VERSION.SDK_INT >= Build.VERSION_CODES.O)
          menuItem.setTooltipText(item.tooltip!!)
      }
    }
  }

  /* Enter the items in this context menu to MENU.
     Assume that MENU will be displayed in VIEW; this may lead to
     popupMenu being called on VIEW if a submenu is selected.

     If MENU is a ContextMenu, set its header title to the one
     contained in this object.  */

  fun expandTo(menu: Menu, view: EmacsView) {
    inflateMenuItems(menu, view)

    /* See if menu is a ContextMenu and a title is set.  */
    if (title == null || menu !is ContextMenu)
      return

    /* Set its title to this.title.  */
    menu.setHeaderTitle(title)
  }

  /* Return the parent or NULL.  */

  fun parent(): EmacsContextMenu? {
    return this.parent
  }

  /* Like display, but does the actual work and runs in the main
     thread.  */

  private fun display1(window: EmacsWindow, xPosition: Int, yPosition: Int): Boolean {
    /* Set this flag to false.  It is used to decide whether or not to
       send 0 in response to the context menu being closed.  */
    itemAlreadySelected = false

    /* No submenu has been selected yet.  */
    wasSubmenuSelected = -1

    return window.view.popupMenu(this, xPosition, yPosition,
                                 false)
  }

  /* Display this context menu on WINDOW, at xPosition and yPosition.
     SERIAL is a number that will be returned in any menu event
     generated to identify this context menu.  */

  fun display(window: EmacsWindow, xPosition: Int,
              yPosition: Int, serial: Int): Boolean {
    val task: FutureTask<Boolean>

    /* Android will permanently cease to display any popup menus at
       all if the list of menu items is empty.  Prevent this by
       promptly returning if there are no menu items.  */

    if (menuItems!!.isEmpty())
      return false

    task = FutureTask<Boolean>(Callable<Boolean> {
      val rc: Boolean

      lastMenuEventSerial = serial
      rc = display1(window, xPosition, yPosition)

      /* Android 3.0 to Android 7.0 perform duplicate calls to
         onContextMenuClosed the second time a context menu is
         dismissed.  Since the second call after such a dismissal is
         otherwise liable to prematurely cancel any context menu
         displayed immediately afterwards, ignore calls received
         within 150 milliseconds of this menu's being displayed.  */

      if (rc && Build.VERSION.SDK_INT >= Build.VERSION_CODES.HONEYCOMB
          && Build.VERSION.SDK_INT < Build.VERSION_CODES.N)
        wasSubmenuSelected = System.currentTimeMillis() - 150

      rc
    })

    return EmacsService.syncRunnable(task)
  }

  /* Dismiss this context menu.  WINDOW is the window where the
     context menu is being displayed.  */

  fun dismiss(window: EmacsWindow) {
    EmacsService.SERVICE!!.runOnUiThread(Runnable {
      window.view.cancelPopupMenu()
      itemAlreadySelected = false
    })
  }
}
