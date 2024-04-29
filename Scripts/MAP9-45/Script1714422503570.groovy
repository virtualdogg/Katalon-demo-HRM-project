import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demoorangeforme-trials712.orangehrmlive.com/')

WebUI.click(findTestObject('Object Repository/input_Login_txtUsername'))

WebUI.setText(findTestObject('Object Repository/input_Login_txtUsername'), 'Admin')

WebUI.setEncryptedText(findTestObject('Object Repository/input_Login_txtPassword'), 'Ns/gyzxDMr4pu6LNweHOow==')


WebUI.click(findTestObject('Object Repository/button_Login'))

WebUI.setText(findTestObject('Object Repository/input_search_menuQuickSearchmenuQuickSearch'), 'education')

WebUI.click(findTestObject('Object Repository/div_Education'))

WebUI.setText(findTestObject('Object Repository/input_search_menuQuickSearchmenuQuickSearch'), 'education')

WebUI.click(findTestObject('Object Repository/i_search'))

WebUI.click(findTestObject('Object Repository/i_add'))

WebUI.click(findTestObject('Object Repository/label_Level of Education'))

// Random text generated from custom keyword:
randomText = CustomKeywords.'banom.textor.createRandomString'()

WebUI.setText(findTestObject('Object Repository/input_Add Education_name'), randomText)
WebUI.takeScreenshot('C:\\Screenshot\\a.png')

// WebUI.setText(findTestObject('Object Repository/input_Add Education_name'), 'dad')
WebUI.click(findTestObject('Object Repository/a_Cancel'))

WebUI.takeScreenshot('C:\\Screenshot\\b.png')
//WebUI.closeBrowser()

