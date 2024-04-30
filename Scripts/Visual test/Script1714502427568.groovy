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

WebUI.setText(findTestObject('Object Repository/Add education/Page_OrangeHRM/input_Login_txtUsername'), 'admin')

WebUI.setEncryptedText(findTestObject('Object Repository/Add education/Page_OrangeHRM/input_Login_txtPassword'), 'Ns/gyzxDMr4pu6LNweHOow==')

WebUI.click(findTestObject('Object Repository/Add education/Page_OrangeHRM/button_Login'))

WebUI.setText(findTestObject('Object Repository/Add education/Page_Employee Management/input_search_menuQuickSearchmenuQuickSearch'), 
    'education')

WebUI.click(findTestObject('Object Repository/Add education/Page_Employee Management/div_Education'))

WebUI.click(findTestObject('Object Repository/Add education/Page_Qualifications/i_add'))

WebUI.click(findTestObject('Object Repository/Add education/Page_Qualifications/label_Level of Education'))

WebUI.setText(findTestObject('Object Repository/Add education/Page_Qualifications/input_Add Education_name'), 'Visual check')

WebUI.click(findTestObject('Object Repository/Add education/Page_Qualifications/a_Save'))

WebUI.click(findTestObject('Object Repository/Add education/Page_Qualifications/div_Successfully Saved'))

WebUI.rightClick(findTestObject('Object Repository/Add education/Page_Qualifications/div_Successfully Saved'))

WebUI.click(findTestObject('Object Repository/Add education/Page_Qualifications/div_more_horizSelect AllLevel of Education _f91407'))

WebUI.takeScreenshotAsCheckpoint('Add edu confirmation')

