<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Login                                  _89807f</name>
   <tag></tag>
   <elementGuidId>7948633a-e3c5-4233-bb23-0e7ca5db08fc</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.login-form-container</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='divLogin']/div[2]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>div >> internal:has-text=&quot;Login Keep me logged in for 30 days Login&quot;i >> nth=3</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>2b668f5c-a4f0-4428-af28-1e02673f3984</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>login-form-container</value>
      <webElementGuid>1bbc035d-ca08-41de-83c5-47eab7dbde76</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
            
                
            
            
                Login            
            
                                
                
                    
                                    
                
                    
                                                                
                            
                            
                        
                                    
                                
                    
                                                Keep me logged in for 30 days                        
                            
                                
                            
                        
                    
                
                                
                    Login
                
            
            
                
                    

                

                

    .alternative-login {
        color: #64728c;
        font-family: 'Nunito';
        border-top: 1px solid #e8eaef;
    }
    
    .alternative-login .alternative-login-description {
        padding: 0.7rem 0;
        font-size: 0.7rem;
        text-align: center;
    }

    .alternative-login .oauth-providers {
        text-align: center;
        display: flex;
        justify-content: center;
    }

    .alternative-login .oauth-providers img.icon {
        height: 2rem;
        width: 2rem;
        max-height: 2rem;
        max-width: 2rem;
        border-radius: 50%;
    }

    .alternative-login .oauth-providers .recent-login {
        position: relative;
        background-color: #eef0f4;
        border-radius: 1.2rem;
        height: 2rem;
        width: 40%;
        padding: 0.2rem;
        display: inline-block;
        bottom: 0.2rem;
    }

    .alternative-login .oauth-providers .recent-login img.icon,
    .alternative-login .oauth-providers .recent-login .placeholder-icon {
        position: absolute;
        left: 0.3rem;
    }

    .alternative-login .oauth-providers .recent-login .recent-login-description {
        display: inline-block;
        font-size: 0.6rem;
        text-align: left;
        position: absolute;
        left: 2.8rem;
        top: 0.4rem;
    }

    .alternative-login .oauth-providers .recent-login .recent-login-description .description-heading,
    .alternative-login .oauth-providers .recent-login .recent-login-description .recent-provider-name {
        max-width: 4.5rem;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        display: block;
    }

    .alternative-login .oauth-providers .recent-login + a.alt-login-link {
        border-left: 1px solid #e8eaef;
        padding-left: 0.5rem;
        margin-left: 0.5rem;
    }

    .alt-login-link,.recent-login {
        text-decoration: none;
        color: inherit;
    }

    .oauth-providers > .alt-login-link {
        height: 2rem;
        position: relative;
        margin-left: 0.5rem;
    }

    .placeholder-icon, .alt-login-link:link, .alt-login-link:visited {
        color: white;
    }
    
    .placeholder-icon {
        display: inline-block;
        border-radius: 50%;
        height: 2rem;
        width: 2rem;
        line-height: 2rem;
        background-color: #2d969b;
        font-weight: 700;
    }
    
    .placeholder-icon.no-logo-index-2 {
        background-color: #eb1e4b;
    }
    
    .placeholder-icon.no-logo-index-3 {
        background-color: #05d7a0;
    }
    
    .placeholder-icon.no-logo-index-4 {
        background-color: #f5dc50;
    }


    $(document).ready(function() {
        $('a.recent-login, a.alt-login-link').click(function(e) {

            if (document.cookie.indexOf('redirect_to_after_login=') === -1) {
                document.cookie = 'redirect_to_after_login' + '=' + encodeURIComponent(window.location.href) + ';path=/;secure';
            }
        });
    });
            
        
    </value>
      <webElementGuid>c8b7cb09-3e25-427d-8693-5e5e8eec93f4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;divLogin&quot;)/div[@class=&quot;login-form-container&quot;]</value>
      <webElementGuid>8cb60ec1-d484-4db1-a233-0a195ca33a64</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='divLogin']/div[2]</value>
      <webElementGuid>d8fd20a5-c6d7-4690-b1be-9c004f962ae2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div/div[2]</value>
      <webElementGuid>30de8365-3132-4e9c-ba8c-40f5b0082d7b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
        
            
                
            
            
                Login            
            
                                
                
                    
                                    
                
                    
                                                                
                            
                            
                        
                                    
                                
                    
                                                Keep me logged in for 30 days                        
                            
                                
                            
                        
                    
                
                                
                    Login
                
            
            
                
                    

                

                

    .alternative-login {
        color: #64728c;
        font-family: &quot; , &quot;'&quot; , &quot;Nunito&quot; , &quot;'&quot; , &quot;;
        border-top: 1px solid #e8eaef;
    }
    
    .alternative-login .alternative-login-description {
        padding: 0.7rem 0;
        font-size: 0.7rem;
        text-align: center;
    }

    .alternative-login .oauth-providers {
        text-align: center;
        display: flex;
        justify-content: center;
    }

    .alternative-login .oauth-providers img.icon {
        height: 2rem;
        width: 2rem;
        max-height: 2rem;
        max-width: 2rem;
        border-radius: 50%;
    }

    .alternative-login .oauth-providers .recent-login {
        position: relative;
        background-color: #eef0f4;
        border-radius: 1.2rem;
        height: 2rem;
        width: 40%;
        padding: 0.2rem;
        display: inline-block;
        bottom: 0.2rem;
    }

    .alternative-login .oauth-providers .recent-login img.icon,
    .alternative-login .oauth-providers .recent-login .placeholder-icon {
        position: absolute;
        left: 0.3rem;
    }

    .alternative-login .oauth-providers .recent-login .recent-login-description {
        display: inline-block;
        font-size: 0.6rem;
        text-align: left;
        position: absolute;
        left: 2.8rem;
        top: 0.4rem;
    }

    .alternative-login .oauth-providers .recent-login .recent-login-description .description-heading,
    .alternative-login .oauth-providers .recent-login .recent-login-description .recent-provider-name {
        max-width: 4.5rem;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        display: block;
    }

    .alternative-login .oauth-providers .recent-login + a.alt-login-link {
        border-left: 1px solid #e8eaef;
        padding-left: 0.5rem;
        margin-left: 0.5rem;
    }

    .alt-login-link,.recent-login {
        text-decoration: none;
        color: inherit;
    }

    .oauth-providers > .alt-login-link {
        height: 2rem;
        position: relative;
        margin-left: 0.5rem;
    }

    .placeholder-icon, .alt-login-link:link, .alt-login-link:visited {
        color: white;
    }
    
    .placeholder-icon {
        display: inline-block;
        border-radius: 50%;
        height: 2rem;
        width: 2rem;
        line-height: 2rem;
        background-color: #2d969b;
        font-weight: 700;
    }
    
    .placeholder-icon.no-logo-index-2 {
        background-color: #eb1e4b;
    }
    
    .placeholder-icon.no-logo-index-3 {
        background-color: #05d7a0;
    }
    
    .placeholder-icon.no-logo-index-4 {
        background-color: #f5dc50;
    }


    $(document).ready(function() {
        $(&quot; , &quot;'&quot; , &quot;a.recent-login, a.alt-login-link&quot; , &quot;'&quot; , &quot;).click(function(e) {

            if (document.cookie.indexOf(&quot; , &quot;'&quot; , &quot;redirect_to_after_login=&quot; , &quot;'&quot; , &quot;) === -1) {
                document.cookie = &quot; , &quot;'&quot; , &quot;redirect_to_after_login&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot; + encodeURIComponent(window.location.href) + &quot; , &quot;'&quot; , &quot;;path=/;secure&quot; , &quot;'&quot; , &quot;;
            }
        });
    });
            
        
    &quot;) or . = concat(&quot;
        
            
                
            
            
                Login            
            
                                
                
                    
                                    
                
                    
                                                                
                            
                            
                        
                                    
                                
                    
                                                Keep me logged in for 30 days                        
                            
                                
                            
                        
                    
                
                                
                    Login
                
            
            
                
                    

                

                

    .alternative-login {
        color: #64728c;
        font-family: &quot; , &quot;'&quot; , &quot;Nunito&quot; , &quot;'&quot; , &quot;;
        border-top: 1px solid #e8eaef;
    }
    
    .alternative-login .alternative-login-description {
        padding: 0.7rem 0;
        font-size: 0.7rem;
        text-align: center;
    }

    .alternative-login .oauth-providers {
        text-align: center;
        display: flex;
        justify-content: center;
    }

    .alternative-login .oauth-providers img.icon {
        height: 2rem;
        width: 2rem;
        max-height: 2rem;
        max-width: 2rem;
        border-radius: 50%;
    }

    .alternative-login .oauth-providers .recent-login {
        position: relative;
        background-color: #eef0f4;
        border-radius: 1.2rem;
        height: 2rem;
        width: 40%;
        padding: 0.2rem;
        display: inline-block;
        bottom: 0.2rem;
    }

    .alternative-login .oauth-providers .recent-login img.icon,
    .alternative-login .oauth-providers .recent-login .placeholder-icon {
        position: absolute;
        left: 0.3rem;
    }

    .alternative-login .oauth-providers .recent-login .recent-login-description {
        display: inline-block;
        font-size: 0.6rem;
        text-align: left;
        position: absolute;
        left: 2.8rem;
        top: 0.4rem;
    }

    .alternative-login .oauth-providers .recent-login .recent-login-description .description-heading,
    .alternative-login .oauth-providers .recent-login .recent-login-description .recent-provider-name {
        max-width: 4.5rem;
        white-space: nowrap;
        text-overflow: ellipsis;
        overflow: hidden;
        display: block;
    }

    .alternative-login .oauth-providers .recent-login + a.alt-login-link {
        border-left: 1px solid #e8eaef;
        padding-left: 0.5rem;
        margin-left: 0.5rem;
    }

    .alt-login-link,.recent-login {
        text-decoration: none;
        color: inherit;
    }

    .oauth-providers > .alt-login-link {
        height: 2rem;
        position: relative;
        margin-left: 0.5rem;
    }

    .placeholder-icon, .alt-login-link:link, .alt-login-link:visited {
        color: white;
    }
    
    .placeholder-icon {
        display: inline-block;
        border-radius: 50%;
        height: 2rem;
        width: 2rem;
        line-height: 2rem;
        background-color: #2d969b;
        font-weight: 700;
    }
    
    .placeholder-icon.no-logo-index-2 {
        background-color: #eb1e4b;
    }
    
    .placeholder-icon.no-logo-index-3 {
        background-color: #05d7a0;
    }
    
    .placeholder-icon.no-logo-index-4 {
        background-color: #f5dc50;
    }


    $(document).ready(function() {
        $(&quot; , &quot;'&quot; , &quot;a.recent-login, a.alt-login-link&quot; , &quot;'&quot; , &quot;).click(function(e) {

            if (document.cookie.indexOf(&quot; , &quot;'&quot; , &quot;redirect_to_after_login=&quot; , &quot;'&quot; , &quot;) === -1) {
                document.cookie = &quot; , &quot;'&quot; , &quot;redirect_to_after_login&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;=&quot; , &quot;'&quot; , &quot; + encodeURIComponent(window.location.href) + &quot; , &quot;'&quot; , &quot;;path=/;secure&quot; , &quot;'&quot; , &quot;;
            }
        });
    });
            
        
    &quot;))]</value>
      <webElementGuid>10a0e000-53b8-4f39-a98d-1a782231d68c</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
