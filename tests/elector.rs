
#[cfg(test)]
mod tests {
  use libcne::cne::Elector;
  use libcne::cne::Citizenship;

  #[test]
  fn test_from_string() {
    let html = r#"
      <!--<a href="../../registro_electoral/impugnacionrenovacion.pdf" target="_blank"> DESCARGA LA PLANILLA</a>-->

      <table width="0" bgcolor="\#90c1e2" border="0" cellpadding="1" cellspacing="1">
      <tr>
        <td>
          <table width="-4" bgcolor="\#ffffff" border="0" cellpadding="0" cellspacing="0">
          <tr>
            <td>
                      <table class="re_titulo_contenido">
              <tr>
                <td class="re_titulo_contenido_texto">REGISTRO ELECTORAL - CONSULTA DE DATOS</td>
                <td align="center" width="30"><a href="javascript:ocultar_consulta_re();"><img src="/web/imagen/cerrar.png" border="0" width="20"></a></td>
              </tr>
              <tr><td class="re_titulo_contenido_linea" colspan="10"/></td></tr>
              </table>
                    </td>
          </tr>
          <tr><td height="10"></td></tr>
          <tr>
            <td>
              <tr width="100%" border="0" cellspacing="0" cellpadding="0">
              <tr>
                <td>
                <table width="100%" border="0" cellspacing="0" cellpadding="0">
      <tr>
        <td colspan="2" bgcolor="\#00387b" height="28" align="center"><font color="\#FFFFFF"><b>DATOS DEL ELECTOR</b></font></td>
      </tr>
      <tr>
        <td colspan="2">
          <table cellpadding="2" width="530">
          <tr>
            <td align="left"><b><font color="\#00387b">Cédula:</font></b></td>
            <td align="left">V-19255544</td>
          </tr>
          <tr>
            <td align="left"><b><font color="\#00387b">Nombre:</font></b></td>
            <td align="left"><b>LEDZE PELIN GARCIA TOVAR</b></td>
          </tr>
          <tr>
            <td align="left"><b><font color="\#00387b">Estado:</font></b></td>
            <td align="left">DTTO. CAPITAL</td>
          </tr>
          <tr>
            <td align="left"><b><font color="\#00387b">Municipio:</font></b></tdsta['descripcion'] = $s_res['descripcion'];>
            <td align="left">CE. BLVNO LIBERTADOR</td>
          </tr>
          <tr>
            <td align="left"><b><font color="\#00387b">Parroquia:</font></b></td>
            <td align="left">PQ. EL RECREO</td>
          </tr>
          <tr>
            <td align="left"><b><font color="\#00387b">Centro:</font></b></td>
            <td align="left"><font color="\#0000FF">UNIDAD EDUCATIVA STELLA MATUTINA</font></td>
          </tr>
          <tr>
            <td align="left"><b><font color="\#00387b">Dirección:</font></b></td>
            <td align="left"><font color="\#0000FF">URBANIZACIÓN AVILA FRENTE AVENIDA COROMOTO. IZQUIERDA TRANSVERSAL SAN JOSE A DOS CUADRAS DE LA CAUCHERA FIRESTONE EDIFICIO</font></td>
          </tr>
          
              
          
              </table>
          
              
              

          <table cellpadding="2" width="530">
              
                      



        <tr>
          <!--<td colspan="2" bgcolor="\#00387b" height="28" width="230" align="center"><font color="\#FFFFFF"><b> VOTA en la Elección Constituyente </b></font></td>-->
        </tr>

        <tr>
          <!--<td align="left"><font color="\#FF0000"> <center> TERRITORIAL </center></font> </td>-->
        </tr>




      </table>
              <table align="center" width="100%" bgcolor="\#ffffff">
              
              

              <tr><td><hr></td></tr>
              <tr>
                <td align="center" colspan="2"><b>Registro Electoral</br>Corte al 31 de Marzo 2020</b></td>
              </tr>
              <br>
      <tr><td><hr></td></tr>
      <tr>
      <td align="center" colspan="2"></td>

      </tr>

              </table>
            </td>
          </tr>
          
        
                  <tr><td colspan="10"><hr></td></tr>
          <tr bgcolor="\#e4ebf3">
            <td colspan="10" align="center">
              <table>
              <tr align="right">
                <td><a href="/web/registro_electoral/imprimir_datos_elector.php?nacionalidad=V&cedula=19255544" target="_blank"><img src="/web/imagen/impresora.png" border="0"></a></td>
                <td valign="middle"><a href="/web/registro_electoral/imprimir_datos_elector.php?nacionalidad=V&cedula=19255544" target="_blank">Imprimir</a></td>
                
                <td width="50"></td>
                
                <td><a href="javascript:ocultar_consulta_re();"><img src="/web/imagen/cerrar.png" border="0"></a></td>
                <td valign="middle"><a href="javascript:ocultar_consulta_re();">Cerrar</a></td>
              </tr>
              </table>
            </td>
          </tr>
              </table>
        </td>
      </tr>
      </table>
      "#;

    let elector = Elector::from(html.to_string());
    assert_eq!(elector.citizenship, Citizenship::V);
    assert_eq!(elector.identity, String::from("19255544"));
    assert_eq!(elector.name, String::from("LEDZE PELIN GARCIA TOVAR"));
    assert_eq!(elector.state, String::from("DTTO. CAPITAL"));
    assert_eq!(elector.town, String::from("CE. BLVNO LIBERTADOR"));
    assert_eq!(elector.parish, String::from("PQ. EL RECREO"));
    assert_eq!(
      elector.voting_center,
      String::from("UNIDAD EDUCATIVA STELLA MATUTINA")
    );
    assert_eq!(
      elector.address, 
      String::from("URBANIZACIÓN AVILA FRENTE AVENIDA COROMOTO. IZQUIERDA TRANSVERSAL SAN JOSE A DOS CUADRAS DE LA CAUCHERA FIRESTONE EDIFICIO")
    );
  }
#[test]
  fn test_fail_from_string() {
    let html = r#"
    
    <!--<a href="../../registro_electoral/impugnacionrenovacion.pdf" target="_blank"> DESCARGA LA PLANILLA</a>-->

    <table width="0" bgcolor="\#90c1e2" border="0" cellpadding="1" cellspacing="1">
    <tr>
      <td>
        <table width="-4" bgcolor="\#ffffff" border="0" cellpadding="0" cellspacing="0">
        <tr>
          <td>
                    <table class="re_titulo_contenido">
            <tr>
              <td class="re_titulo_contenido_texto">REGISTRO ELECTORAL - CONSULTA DE DATOS</td>
              <td align="center" width="30"><a href="javascript:ocultar_consulta_re();"><img src="/web/imagen/cerrar.png" border="0" width="20"></a></td>
            </tr>
            <tr><td class="re_titulo_contenido_linea" colspan="10"/></td></tr>
            </table>
                  </td>
        </tr>
        <tr><td height="10"></td></tr>
        <tr>
          <td>
            <tr width="100%" border="0" cellspacing="0" cellpadding="0">
            <tr>
              <td>
                  <table width="100%" border="0" cellspacing="0" cellpadding="0">
        <tr>
          <td colspan="2" bgcolor="\#00387b" height="28" align="center"><font color="\#FFFFFF"><strong>DATOS PERSONALES</strong></font></td>
        </tr>
        <tr>
          <td colspan="2">
            <table align="center" cellpadding="2">
            <tr>
              <td colspan="3"><strong><font color="\#00387b">Cédula:</font></strong> V-3242323</td>
            </tr>
                    <tr>
              <td colspan="3"><strong><font color="\#ff0000">El número de cédula ingresado no corresponde a un elector</font></strong></td>
            </tr>
                    </table>
          </td>
        </tr>
        <tr>
          <td colspan="2" bgcolor="\#00387b" height="28" align="center"><font color="\#FFFFFF"><strong>ESTATUS</strong></font></td>
        </tr>
        <tr>
          <td colspan="2">
            <table align="right">
            <tr>
              <td>Esta cédula de identidad presenta una objeción por lo que no podrá ejercer su derecho al voto.</td>
            </tr>
            <tr>
              <td align="center"><a href="http://www.cne.gob.ve/web/documentos/pdf/re/planilla_reclamo_RE_2014.pdf"> <font color=#666699>Planilla de Reclamo y Registro de Fallecidos</font></a></td>
            </tr>
            <tr>
              <td align="center"><a href="http://www.cne.gob.ve/web/documentos/pdf/re/instructivo_planilla_reclamo_RE_2012.pdf"> <font color=#666699>Instructivo para llenar la Planilla de Reclamo y Registro de Fallecidos</font></a></td>
            </tr>
            </table>
          </td>
        </tr>
        <tr>
          <td colspan="2" bgcolor="\#00387b" height="28" align="center"><font color="\#FFFFFF"><strong>DESCRIPCIÓN DE LA OBJECIÓN</strong></font></td>
        </tr>
        <tr>
          <td colspan="2">
            <table align="right">
            <tr>
              <td><strong><font color="\#00387b">Objeción:</font></strong> FALLECIDO (3)</td>
            </tr>
            <tr>
              <td><strong><font color="\#00387b">Descripción:</font></strong> Es el status que se le asigna a una electora o un elector ya fallecido. </td>
            </tr>
            <tr>
              <td><strong><font color="\#00387b">Institución donde debe solventar la objeción:</font></strong> CNE (Consejo Nacional Electoral): Dirección de  Información al Elector, Oficina  Regional Electoral, Centros de Actualización</td>
            </tr>
            <tr>
              <td><strong><font color="\#00387b">Requisitos a presentar ante el Consejo Nacional Electoral (Oficina Regional Electoral, Centro de Actualizacion, Oficina de Informacion al Elector):</font></strong> 1.- Copia de la cédula de identidad. 2.- Consignar al menos uno de los siguientes documentos: A- Fe de vida en original expedida por las Oficinas o unidades de Registro Civil de su lugar de residencia. B- Declaración jurada (según formato página web).  C- Oficio o carta de reclamo con exposición de motivo</td>
            </tr>
                    <tr>
              <td><a href="http://www.cne.gob.ve/web/documentos/pdf/Declaracion_Jurada.pdf" target="_blank"><strong>Declaracion Jurada</strong></a>.</td>
            </tr>
                    </table>
          </td>
        </tr>
        </table>
              <table align="center" width="100%" bgcolor="\#ffffff">
            
            

            <tr><td><hr></td></tr>
            <tr>
              <td align="center" colspan="2"><b>Registro Electoral</br>Corte al 31 de Marzo 2020</b></td>
            </tr>
            <br>
    <tr><td><hr></td></tr>
    <tr>
    <td align="center" colspan="2"></td>

    </tr>

            </table>
          </td>
        </tr>
        
      
                <tr><td colspan="10"><hr></td></tr>
        <tr bgcolor="\#e4ebf3">
          <td colspan="10" align="center">
            <table>
            <tr align="right">
              <td><a href="/web/registro_electoral/imprimir_datos_elector.php?nacionalidad=V&cedula=3242323" target="_blank"><img src="/web/imagen/impresora.png" border="0"></a></td>
              <td valign="middle"><a href="/web/registro_electoral/imprimir_datos_elector.php?nacionalidad=V&cedula=3242323" target="_blank">Imprimir</a></td>
              
              <td width="50"></td>
              
              <td><a href="javascript:ocultar_consulta_re();"><img src="/web/imagen/cerrar.png" border="0"></a></td>
              <td valign="middle"><a href="javascript:ocultar_consulta_re();">Cerrar</a></td>
            </tr>
            </table>
          </td>
        </tr>
            </table>
      </td>
    </tr>
    </table>

    "#;
     let result = std::panic::catch_unwind(|| Elector::from(html.to_string()));
    assert!(result.is_err());
  }
}
